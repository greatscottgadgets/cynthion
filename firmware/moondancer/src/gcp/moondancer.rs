//! Implementation for the GCP `moondancer` class.

use log::{debug, error, trace, warn};
use zerocopy::byteorder::{LittleEndian, U16, U32};
use zerocopy::{FromBytes, FromZeroes, Unaligned};

use crate::{hal, pac};
use hal::smolusb;
use pac::csr::interrupt;

use smolusb::device::Speed;
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, SetupPacket};
use smolusb::traits::{
    ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
};

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{
    self, iter_to_response, GreatDispatch, GreatResponse, Verb, LIBGREAT_MAX_COMMAND_SIZE,
};

use crate::debug::Bit;
use ladybug::Channel;

// - types --------------------------------------------------------------------

/// USB quirk flags
#[allow(non_snake_case, non_upper_case_globals)]
pub mod QuirkFlag {
    pub const SetAddressManually: u16 = 0x0001;
}

struct Packet {
    endpoint_number: u8,
    bytes_read: usize,
    buffer: [u8; smolusb::EP_MAX_PACKET_SIZE],
}

impl Packet {
    const fn new(endpoint_number: u8, bytes_read: usize) -> Self {
        Self {
            endpoint_number,
            bytes_read,
            buffer: [0; smolusb::EP_MAX_PACKET_SIZE],
        }
    }
}

// - Moondancer --------------------------------------------------------------

use heapless::spsc::Queue;
use heapless::Vec;

/// Moondancer
pub struct Moondancer {
    usb0: hal::Usb0,
    quirk_flags: u16,
    ep_in_max_packet_size: [u16; smolusb::EP_MAX_ENDPOINTS],
    ep_out_max_packet_size: [u16; smolusb::EP_MAX_ENDPOINTS],
    irq_queue: Queue<UsbEvent, 64>,
    control_queue: Queue<SetupPacket, 8>,
    packet_buffer: Vec<Packet, 4>,
    pending_set_address: Option<u8>,
}

impl Moondancer {
    #[must_use]
    pub fn new(usb0: hal::Usb0) -> Self {
        Self {
            usb0,
            quirk_flags: 0,
            ep_in_max_packet_size: [0; smolusb::EP_MAX_ENDPOINTS],
            ep_out_max_packet_size: [0; smolusb::EP_MAX_ENDPOINTS],
            irq_queue: Queue::new(),
            control_queue: Queue::new(),
            packet_buffer: Vec::new(),
            pending_set_address: None,
        }
    }

    pub fn dispatch_event(&mut self, event: UsbEvent) {
        // filter interrupt events
        let event = match event {
            UsbEvent::BusReset => {
                // flush queues, the actual bus reset is handled in the irq handler for lower latency
                //while let Some(_) = self.irq_queue.dequeue() {}
                //while let Some(_) = self.control_queue.dequeue() {}
                self.pending_set_address = None;
                event
            }

            UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet) => {
                // check if it is a SetAddress request and handle it locally for lowest latency
                use smolusb::setup::{Request, RequestType};
                let direction = setup_packet.direction();
                let request_type = setup_packet.request_type();
                let request = setup_packet.request();

                if matches!(
                    (direction, request_type, request),
                    (
                        Direction::HostToDevice,
                        RequestType::Standard,
                        Request::SetAddress
                    )
                ) {
                    // read the address
                    let address: u8 = (setup_packet.value & 0x7f) as u8;

                    // set pending flag to perform set address after SendComplete
                    self.pending_set_address = Some(address);

                    // send ZLP to host to end status stage
                    self.usb0.ack(endpoint_number, Direction::HostToDevice);
                    return;
                }

                // queue setup packet and convert to a control event
                match self.control_queue.enqueue(setup_packet) {
                    Ok(()) => (),
                    Err(_) => {
                        error!("Moondancer - control queue overflow");
                        loop {
                            unsafe {
                                riscv::asm::nop();
                            }
                        }
                    }
                }
                UsbEvent::ReceiveControl(endpoint_number)
            }

            UsbEvent::SendComplete(_endpoint_number) => {
                // catch EP_IN SendComplete after SetAddress ack
                if let Some(address) = self.pending_set_address.take() {
                    self.usb0.set_address(address);
                    return;
                }

                // drop event, because - currently - we're not using it in moondancer.py
                return;
            }

            UsbEvent::ReceivePacket(endpoint_number) => {
                // drain FIFO
                let mut rx_buffer: [u8; smolusb::EP_MAX_PACKET_SIZE] =
                    [0; smolusb::EP_MAX_PACKET_SIZE];
                let bytes_read = self.usb0.read(endpoint_number, &mut rx_buffer);

                // create Packet
                let mut packet = Packet::new(endpoint_number, bytes_read);
                if packet.bytes_read > packet.buffer.len() {
                    error!(
                        "MD moondancer::dispatch_event(ReceivePacket({})) receive buffer overflow {} bytes",
                        packet.endpoint_number, packet.bytes_read
                    );
                    // TODO we can probably do better than truncating the packet
                    packet.bytes_read = packet.buffer.len();
                } else {
                    packet.buffer[..packet.bytes_read]
                        .copy_from_slice(&rx_buffer[..packet.bytes_read]);
                }

                // append to packet buffer
                match self.packet_buffer.push(packet) {
                    Ok(()) => {}
                    Err(_packet) => {
                        error!(
                            "MD moondancer::dispatch_event(ReceivePacket({})) packet buffer overflow",
                            endpoint_number
                        );
                    }
                }

                event
            }

            UsbEvent::ReceiveControl(_) => {
                // no-op, just pass it on through
                event
            }
        };

        // enqueue interrupt event
        match self.irq_queue.enqueue(event) {
            Ok(()) => {}
            Err(_) => {
                error!("Moondancer - irq queue overflow");
            }
        }
    }
}

// - usb0 interrupt handlers --------------------------------------------------

impl Moondancer {
    /// Enable USB events and CPU interrupts for the USB controller.
    ///
    /// # Safety
    ///
    /// This function operates directly on the CPU's Machine IRQ Mask
    /// register. It is not interrupt-safe and any pending interrupts
    /// may be dropped when calling it.
    pub unsafe fn enable_usb_interrupts(&self) {
        interrupt::enable(pac::Interrupt::USB0);
        interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB0_EP_IN);
        interrupt::enable(pac::Interrupt::USB0_EP_OUT);

        // enable all usb events
        self.usb0.enable_events();
    }

    /// Disable USB events and CPU interrupts for the USB controller.
    ///
    /// # Safety
    ///
    /// This function operates directly on the CPU's Machine IRQ Mask
    /// register. It is not interrupt-safe and any pending interrupts
    /// may be dropped when calling it.
    pub unsafe fn disable_usb_interrupts(&self) {
        // disable all usb events
        self.usb0.disable_events();

        interrupt::disable(pac::Interrupt::USB0);
        interrupt::disable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::disable(pac::Interrupt::USB0_EP_IN);
        interrupt::disable(pac::Interrupt::USB0_EP_OUT);
    }
}

// - verb implementations: device connection ----------------------------------

impl Moondancer {
    /// Connect the USB interface.
    pub fn connect(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            ep0_max_packet_size: U16<LittleEndian>,
            device_speed: u8,
            quirk_flags: U16<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let ep0_max_packet_size = args.ep0_max_packet_size.into();
        let device_speed = Speed::from_libusb(args.device_speed);
        let quirk_flags = args.quirk_flags.into();

        self.ep_in_max_packet_size[0] = ep0_max_packet_size;
        self.ep_out_max_packet_size[0] = ep0_max_packet_size;
        self.quirk_flags = quirk_flags;

        // connect usb0 device and enable interrupts
        self.usb0.connect(device_speed);
        unsafe { self.enable_usb_interrupts() };

        // wait for things to settle and get connection speed
        unsafe {
            riscv::asm::delay(20_000_000);
        }
        let speed: Speed = self.usb0.device.status().read().speed().bits().into();

        log::info!("Moondancer connected {:?}-speed device to host.", speed);

        log::debug!(
            "MD moondancer::connect(ep0_max_packet_size:{}, device_speed:{:?}, quirk_flags:{}) -> {:?}",
            args.ep0_max_packet_size, device_speed, args.quirk_flags, speed
        );

        Ok([].into_iter())
    }

    /// Terminate all existing communication and disconnects the USB interface.
    pub fn disconnect(&mut self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        // disable interrupts and disconnect USB interface
        unsafe { self.disable_usb_interrupts() };
        self.usb0.disconnect();

        // reset connection state
        self.quirk_flags = 0;
        self.ep_in_max_packet_size = [0; smolusb::EP_MAX_ENDPOINTS];
        self.ep_out_max_packet_size = [0; smolusb::EP_MAX_ENDPOINTS];
        self.pending_set_address = None;

        // flush queues
        while self.irq_queue.dequeue().is_some() {}
        while self.control_queue.dequeue().is_some() {}

        // clear quirk flags
        self.quirk_flags = 0;

        log::info!("Moondancer disconnected");

        Ok([].into_iter())
    }

    /// Perform a USB bus reset.
    pub fn bus_reset(&mut self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        // We sent the event to facedancer but the actual bus reset already happened locally
        // in the interrupt handler.

        debug!("MD moondancer::bus_reset()");

        Ok([].into_iter())
    }
}

// - verb implementations: status & control -----------------------------------

impl Moondancer {
    /// Returns the earliest control packet in the queue.
    pub fn read_control(&mut self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let Some(setup_packet) = self.control_queue.dequeue() else {
            error!("Moondancer - no packets in control queue");
            loop {
                unsafe {
                    riscv::asm::nop();
                }
            }
        };

        debug!("MD moondancer::read_control() -> {:?}", setup_packet);

        Ok(SetupPacket::as_bytes(setup_packet).into_iter())
    }

    /// Set the device address.
    pub fn set_address(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            address: u8,
            deferred: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let address = args.address & 0x7f;

        // TODO handle
        let _deferred = args.deferred != 0;

        // activate new address
        self.usb0.set_address(address);

        // ack status
        self.usb0.ack(0, Direction::HostToDevice);

        trace!(
            "MD moondancer::set_address(address:{}, deferred:{})",
            args.address,
            args.deferred
        );

        Ok([].into_iter())
    }

    /// Configure endoints.
    pub fn configure_endpoints(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(Debug, FromBytes, FromZeroes, Unaligned)]
        struct ArgEndpoint {
            address: u8,
            max_packet_size: U16<LittleEndian>,
            transfer_type: u8,
        }

        log::debug!("MD moondancer::configure_endpoints()");

        // while we have endpoint triplets to handle
        let mut byte_slice = arguments;
        while let Some((endpoint, next)) =
            zerocopy::Ref::<_, ArgEndpoint>::new_from_prefix(byte_slice)
        {
            let endpoint_number = endpoint.address & 0x7f;

            log::debug!(
                "  moondancer::configure_endpoint(0x{:x}) -> {} -> max_packet_size:{}",
                endpoint.address,
                endpoint_number,
                endpoint.max_packet_size
            );
            byte_slice = next;

            // endpoint zero is always the control endpoint, and can't be configured
            if endpoint_number == 0x00 {
                warn!(
                    "  ignoring request to reconfigure control endpoint address: 0x{:x}",
                    endpoint.address
                );
                continue;
            }

            // ignore endpoint configurations we won't be able to handle
            if endpoint.max_packet_size.get() as usize > smolusb::EP_MAX_PACKET_SIZE {
                error!(
                    "  failed to configure endpoint address 0x{:x} with max packet size {} > {}",
                    endpoint.address,
                    endpoint.max_packet_size,
                    smolusb::EP_MAX_PACKET_SIZE,
                );
                return Err(GreatError::InvalidArgument);
            }

            // configure endpoint max packet sizes
            if Direction::from(endpoint.address) == Direction::HostToDevice {
                self.ep_out_max_packet_size[endpoint_number as usize] =
                    endpoint.max_packet_size.into();
            } else {
                self.ep_in_max_packet_size[endpoint_number as usize] =
                    endpoint.max_packet_size.into();
            }

            // prime any OUT endpoints
            if Direction::from(endpoint.address) == Direction::HostToDevice {
                log::debug!(
                    "  priming HostToDevice (OUT) endpoint address: {}",
                    endpoint.address
                );
                self.usb0.ep_out_prime_receive(endpoint_number);
            }
        }

        let iter = [].into_iter();
        Ok(iter)
    }

    /// Stall the given USB IN endpoint number.
    pub fn stall_endpoint_in(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_number = args.endpoint_number;

        // stall IN end
        self.usb0.stall_endpoint_in(endpoint_number);

        log::debug!("MD moondancer::stall_endpoint_in({})", args.endpoint_number);

        Ok([].into_iter())
    }

    /// Stall the given USB OUT endpoint number.
    pub fn stall_endpoint_out(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_number = args.endpoint_number;

        // stall OUT end
        self.usb0.stall_endpoint_out(endpoint_number);

        log::debug!(
            "MD moondancer::stall_endpoint_out({})",
            args.endpoint_number
        );

        Ok([].into_iter())
    }

    /// Clears a halt condition on the target endpoint number.
    pub fn clear_feature_endpoint_halt(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint_number: u8,
            direction: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_number = args.endpoint_number;
        let direction = match args.direction {
            0 => Direction::HostToDevice, // OUT
            _ => Direction::DeviceToHost, // IN
        };

        // Clear feature endpoint halt
        self.usb0
            .clear_feature_endpoint_halt(endpoint_number, direction);

        log::debug!(
            "MD moondancer::clear_feature_endpoint_halt({}, {:?})",
            endpoint_number,
            direction
        );

        Ok([].into_iter())
    }
}

// - verb implementations: data transfer --------------------------------------

impl Moondancer {
    pub fn read_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_number = args.endpoint_number;

        let packet = match self
            .packet_buffer
            .iter()
            .position(|packet| packet.endpoint_number == endpoint_number)
        {
            Some(index) => self.packet_buffer.remove(index),
            None => {
                error!(
                    "MD moondancer::read_endpoint({}) has no packet buffered for endpoint",
                    endpoint_number
                );
                // TODO actually handle this case in moondancer.py
                Packet::new(endpoint_number, 0)
            }
        };

        log::debug!(
            "MD moondancer::read_endpoint({}) -> bytes_read:{}",
            packet.endpoint_number,
            packet.bytes_read
        );

        Ok(packet.buffer.into_iter().take(packet.bytes_read))
    }

    pub fn test_read_endpoint(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            payload_length: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let payload_length: usize = u32::from(args.payload_length) as usize;

        log::debug!("MD moondancer::test_read_endpoint({})", payload_length);

        if payload_length > LIBGREAT_MAX_COMMAND_SIZE {
            error!(
                "MD moondancer::test_read_endpoint error overflow: {}",
                payload_length
            );
            return Err(GreatError::NoBufferSpaceAvailable);
        }

        let mut rx_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE] = [0; LIBGREAT_MAX_COMMAND_SIZE];
        #[allow(clippy::cast_possible_truncation)] // seriously clippy?
        for (index, byte) in rx_buffer.iter_mut().enumerate() {
            *byte = (index % usize::from(u8::MAX)) as u8;
        }

        Ok(rx_buffer.into_iter().take(payload_length))
    }

    pub fn ep_out_prime_receive(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        self.usb0.ep_out_prime_receive(args.endpoint_number);

        debug!(
            "MD moondancer::ep_out_prime_receive({})",
            args.endpoint_number
        );

        Ok([].into_iter())
    }

    pub fn ep_out_interface_enable(
        &mut self,
        _arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        // 0. clear receive fifo in case the previous transaction wasn't handled
        if self.usb0.ep_out.status().read().have().bit() {
            log::warn!("Re-enabling interface with unread data: Usb0");
            self.usb0.ep_out.reset().write(|w| w.fifo().bit(true));
        }

        // 1. re-enable ep_out interface
        self.usb0.ep_out.enable().write(|w| w.enabled().bit(true));

        debug!("MD moondancer::ep_out_interface_enable()");

        Ok([].into_iter())
    }

    pub fn write_control_endpoint(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::Ref<B, u8>,
            requested_length: zerocopy::Ref<B, U16<LittleEndian>>,
            blocking: zerocopy::Ref<B, u8>,
            payload: B,
        }

        let (endpoint_number, arguments) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let (requested_length, arguments) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let (blocking, payload) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let args = Args {
            endpoint_number,
            requested_length,
            blocking,
            payload,
        };

        let endpoint_number: u8 = args.endpoint_number.read();
        let requested_length = args.requested_length.read();
        let blocking = args.blocking.read() != 0;
        let payload_length = args.payload.len();
        let iter = args.payload.iter();
        let max_packet_size = self.ep_in_max_packet_size[endpoint_number as usize] as usize;

        let bytes_written = self.usb0.write_with_packet_size(
            endpoint_number,
            Some(requested_length.into()),
            iter.copied().take(requested_length.into()),
            max_packet_size,
        );

        // wait for send to complete if we're blocking
        if blocking && self.usb0.ep_in_busy(endpoint_number, "moondancer::write_control_endpoint()") {
                log::error!(
                    "moondancer::write_control_endpoint timed out after {} bytes during write of {} bytes",
                    payload_length,
                    bytes_written
                );
                return Err(GreatError::StreamIoctlTimeout);
        }

        log::debug!(
            "MD moondancer::write_control_endpoint(endpoint_number:{}, requested_length:{}, blocking:{} payload.len:{} ({})) max_packet_size:{} bytes_written:{}",
            endpoint_number,
            requested_length,
            blocking,
            payload_length,
            args.payload.iter().len(),
            max_packet_size,
            bytes_written,
        );

        Ok([].into_iter())
    }

    #[allow(clippy::too_many_lines)]
    pub fn write_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::Ref<B, u8>,
            blocking: zerocopy::Ref<B, u8>,
            payload: B,
        }
        let (endpoint_number, arguments) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let (blocking, payload) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let args = Args {
            endpoint_number,
            blocking,
            payload,
        };

        let endpoint_number: u8 = args.endpoint_number.read();
        let blocking = args.blocking.read() != 0;
        let payload_length = args.payload.len();
        let iter = args.payload.iter();
        let max_packet_size = self.ep_in_max_packet_size[endpoint_number as usize] as usize;

        // check if ep_in is available
        if self.usb0.ep_in_busy(endpoint_number, "moondancer::write_endpoint()") {
            return Err(GreatError::StreamIoctlTimeout);
        }

        // write data out to EP_IN, splitting into packets of max_packet_size
        let mut bytes_written: usize = 0;
        for byte in iter {
            self.usb0
                .ep_in
                .data()
                .write(|w| unsafe { w.byte().bits(*byte) });
            bytes_written += 1;

            // send data if we've written max_packet_size
            if bytes_written % max_packet_size == 0 {
                unsafe {
                    self.usb0.set_tx_ack_active(endpoint_number);
                }
                self.usb0
                    .ep_in
                    .endpoint()
                    .write(|w| unsafe { w.number().bits(endpoint_number) });

                if self.usb0.ep_in_busy(endpoint_number, "moondancer::write_endpoint()") {
                    log::error!(
                        "moondancer::write_endpoint timed out after {} bytes",
                        bytes_written
                    );
                    return Err(GreatError::StreamIoctlTimeout);
                }
            }
        }

        // finally, prime IN endpoint to either send
        // remaining queued data or a ZLP if the fifo is
        // empty.
        //
        // FIXME this conditional is to work around a problem where
        // Facedancer has taken responsibility for splitting the
        // packets up. We probably need two moondancer write methods
        // to be honest.
        if bytes_written != max_packet_size {
            unsafe {
                self.usb0.set_tx_ack_active(endpoint_number);
            }
            self.usb0
                .ep_in
                .endpoint()
                .write(|w| unsafe { w.number().bits(endpoint_number) });
        }

        // wait for send to complete if we're blocking
        if blocking && self.usb0.ep_in_busy(endpoint_number, "moondancer::write_control_endpoint()") {
            log::error!(
                "moondancer::write_endpoint timed out after {} bytes during write of {} bytes",
                payload_length,
                bytes_written
            );
            return Err(GreatError::StreamIoctlTimeout);
        }

        log::debug!(
            "MD moondancer::write_endpoint(endpoint_number:{}, blocking:{} payload.len:{} ({})) max_packet_size:{} bytes_written:{}",
            endpoint_number,
            blocking,
            payload_length,
            args.payload.iter().len(),
            max_packet_size,
            bytes_written,
        );

        Ok([].into_iter())
    }

    pub fn test_write_endpoint(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::Ref<B, u8>,
            payload: B,
        }
        let (endpoint_number, payload) = zerocopy::Ref::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let args = Args {
            endpoint_number,
            payload,
        };

        let endpoint: u8 = args.endpoint_number.read();
        let payload_length = args.payload.len();

        debug!(
            "MD moondancer::test_write_endpoint(endpoint_number:{}, payload.len:{})",
            endpoint, payload_length,
        );

        Ok(payload_length.to_le_bytes().into_iter())
    }
}

// - verb implementations: interrupts -------------------------------------------

impl Moondancer {
    /// Get the most recent USB driver messages.
    ///
    /// # Return Value
    ///
    /// [(type, interface, endpoint)]
    pub fn get_interrupt_events(
        &mut self,
        _arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        let mut tx_buffer = [0_u8; LIBGREAT_MAX_COMMAND_SIZE];

        let clone = self.irq_queue.clone();
        self.irq_queue = Queue::new();

        let length = clone.len() * 2;
        let response = clone.iter().flat_map(|event| event.into_bytes());

        for (dest, src) in tx_buffer.iter_mut().zip(response) {
            *dest = src;
        }

        Ok(tx_buffer.into_iter().take(length))
    }

    /// Returns test data containing USB driver messages.
    ///
    /// # Return value
    ///
    /// [(type, interface, endpoint)]
    pub fn test_get_interrupt_events(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        debug!("MD moondancer::test_get_interrupt_events()");

        self.irq_queue.enqueue(UsbEvent::BusReset).ok();
        self.irq_queue.enqueue(UsbEvent::ReceiveControl(1)).ok();
        self.irq_queue.enqueue(UsbEvent::ReceivePacket(2)).ok();
        self.irq_queue.enqueue(UsbEvent::SendComplete(3)).ok();

        self.get_interrupt_events(arguments)
    }

    /// Get endpoint NAK status.
    ///
    /// # Return Value
    ///
    /// bitmask
    pub fn get_nak_status(&mut self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let nak_status = (self.usb0.ep_in.status().read().nak().bits() & 0xffff) as u16;
        Ok(nak_status.to_le_bytes().into_iter())
    }
}

// - class information --------------------------------------------------------

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::moondancer,
    name: "moondancer",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "API for fine-grained control of the Target USB port.\0";

/// Verb definitions for class: moondancer
///
/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 19] = [
    // - device connection --
    Verb {
        id: 0x00,
        name: "connect\0",
        doc: "\0", //"Connect the target to the host. device_speed is 3:high, 2:full, 1:low\0",
        in_signature: "<HBH\0",
        in_param_names: "ep0_max_packet_size, device_speed, quirk_flags\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x01,
        name: "disconnect\0",
        doc: "\0", //"Disconnect the target port from the host.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x02,
        name: "bus_reset\0",
        doc: "\0", //"Cause the target device to handle a bus reset.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - status & control --
    Verb {
        id: 0x03,
        name: "read_control\0",
        doc: "\0", //"Read a setup packet from the control endpoint.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<8X\0",
        out_param_names: "setup_packet\0",
    },
    Verb {
        id: 0x04,
        name: "set_address\0",
        doc: "\0", //"Set the address of the target device.\nIf deferred is set this action won't complete until the setup phase ends.\0",
        in_signature: "<BB\0",
        in_param_names: "address, deferred\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x05,
        name: "configure_endpoints\0",
        doc: "\0", //"Set up all of the non-control endpoints for the device.\0",
        in_signature: "<*(BHB)\0",
        in_param_names: "endpoint_descriptors\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x06,
        name: "stall_endpoint_in\0",
        doc: "\0", //"Stall the IN endpoint with the provided endpoint number.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x07,
        name: "stall_endpoint_out\0",
        doc: "\0", //"Stall the OUT endpoint with the provided endpoint number.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x08,
        name: "clear_feature_endpoint_halt\0",
        doc: "\0", //"Clear a halt condition on the target endpoint address.\0",
        in_signature: "<BB\0",
        in_param_names: "endpoint_number, direction\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - data transfer --
    Verb {
        id: 0x09,
        name: "read_endpoint\0",
        doc: "\0", //"Read a packet from an OUT endpoint.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "<*X\0",
        out_param_names: "read_data\0",
    },
    Verb {
        id: 0x0a,
        name: "ep_out_prime_receive\0",
        doc: "\0", //"Prepare OUT endpoint to receive a single packet.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x0b,
        name: "write_control_endpoint\0",
        doc: "\0", //"Write a packet to an IN endpoint.\0",
        in_signature: "<BHB*X\0",
        in_param_names: "endpoint_number, requested_length, blocking, payload\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x0c,
        name: "write_endpoint\0",
        doc: "\0", //"Write a packet to an IN endpoint.\0",
        in_signature: "<BB*X\0",
        in_param_names: "endpoint_number, blocking, payload\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - interrupts --
    Verb {
        id: 0x0d,
        name: "get_interrupt_events\0",
        doc: "\0", //"Return the most recent driver messages.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<*(BB)\0",
        out_param_names: "type, endpoint\0",
    },
    Verb {
        id: 0x0e,
        name: "get_nak_status\0",
        doc: "\0", //"Return endpoint NAK status.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<H\0",
        out_param_names: "bitmask\0",
    },
    Verb {
        id: 0x0f,
        name: "ep_out_interface_enable\0",
        doc: "\0", //"Enable OUT endpoints to resume receiving packets.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - tests --
    Verb {
        id: 0x28,
        name: "test_read_endpoint\0",
        doc: "\0", //"Return read_endpoint with payload_length of test data.\0",
        in_signature: "<I\0",
        in_param_names: "payload_length\0",
        out_signature: "<*X\0",
        out_param_names: "read_data\0",
    },
    Verb {
        id: 0x2a,
        name: "test_write_endpoint\0",
        doc: "\0", //"Write a packet to an IN endpoint and return the length received.\0",
        in_signature: "<B*X\0",
        in_param_names: "endpoint_number, payload\0",
        out_signature: "<I\0",
        out_param_names: "payload_length\0",
    },
    Verb {
        id: 0x2b,
        name: "test_get_interrupt_events\0",
        doc: "\0", //"Return get_interrupt_events() with test data.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<*(BB)\0",
        out_param_names: "type, endpoint\0",
    },
];

// - dispatch -----------------------------------------------------------------

impl GreatDispatch for Moondancer {
    fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse> {
        match verb_number {
            0x00 => {
                // moondancer::connect
                let iter = self.connect(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x01 => {
                // moondancer::disconnect
                let iter = self.disconnect(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x02 => {
                // moondancer::bus_reset
                let iter = self.bus_reset(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x03 => {
                // moondancer::read_control
                ladybug::trace(Channel::A, Bit::A_READ_CONTROL, || {
                    let iter = self.read_control(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x04 => {
                // moondancer::set_address
                let iter = self.set_address(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x05 => {
                // moondancer::configure_endpoints
                let iter = self.configure_endpoints(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x06 => {
                // moondancer::stall_endpoint_in
                let iter = self.stall_endpoint_in(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x07 => {
                // moondancer::stall_endpoint_out
                let iter = self.stall_endpoint_out(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x08 => {
                // moondancer::clear_feature_endpoint_halt
                let iter = self.clear_feature_endpoint_halt(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x09 => {
                // moondancer::read_endpoint
                ladybug::trace(Channel::A, Bit::A_READ_ENDPOINT, || {
                    let iter = self.read_endpoint(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x0a => {
                // moondancer::ep_out_prime_receive
                ladybug::trace(Channel::A, Bit::A_PRIME_RECEIVE, || {
                    let iter = self.ep_out_prime_receive(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x0b => {
                // moondancer::write_control_endpoint
                ladybug::trace(Channel::A, Bit::A_WRITE_ENDPOINT, || {
                    let iter = self.write_control_endpoint(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x0c => {
                // moondancer::write_endpoint
                ladybug::trace(Channel::A, Bit::A_WRITE_ENDPOINT, || {
                    let iter = self.write_endpoint(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x0d => {
                // moondancer::get_interrupt_events
                ladybug::trace(Channel::A, Bit::A_GET_EVENTS, || {
                    let iter = self.get_interrupt_events(arguments)?;
                    let response = iter_to_response(iter, response_buffer);
                    Ok(response)
                })
            }
            0x0e => {
                // moondancer::get_nak_status
                let iter = self.get_nak_status(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x0f => {
                // moondancer::ep_out_interface_enable
                let iter = self.ep_out_interface_enable(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }

            // test APIs
            0x28 => {
                // moondancer::test_read_endpoint
                let iter = self.test_read_endpoint(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x2a => {
                // moondancer::test_write_endpoint
                let iter = self.test_write_endpoint(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x2b => {
                // moondancer::test_get_interrupt_events
                let iter = self.test_get_interrupt_events(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }

            _verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
