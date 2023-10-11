#![allow(dead_code, unused_imports, unused_variables)] // TODO

use core::any::Any;
use core::cell::RefCell;
use core::slice;
use core::{array, iter};

use log::{debug, error, trace, warn};
use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian, Unaligned, U16, U32};

use smolusb::device::{Speed, UsbDevice};
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, RequestType, SetupPacket};
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
    WriteRefEndpoint,
};

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{self, iter_to_response, GreatResponse, Verb, LIBGREAT_MAX_COMMAND_SIZE};

use crate::{hal, pac};
use hal::smolusb;
use pac::csr::interrupt;

// - types --------------------------------------------------------------------

/// QuirkFlags
#[allow(non_snake_case, non_upper_case_globals)]
pub mod QuirkFlag {
    pub const SetAddressManually: u16 = 0x0001;
}

// - Moondancer --------------------------------------------------------------

use crate::event::InterruptEvent; // TODO use smolusb::event::UsbEvent instead
use heapless::spsc::Queue;

/// Moondancer
pub struct Moondancer {
    pub usb0: hal::Usb0, // TODO needs to be private
    pub queue: Queue<InterruptEvent, 64>,
    quirk_flags: u16,
    ep_in_max_packet_size: [u16; crate::EP_MAX_ENDPOINTS],
    ep_out_max_packet_size: [u16; crate::EP_MAX_ENDPOINTS],
}

impl Moondancer {
    pub fn new(usb0: hal::Usb0) -> Self {
        Self {
            usb0,
            queue: Queue::new(),
            quirk_flags: 0,
            ep_in_max_packet_size: [0; crate::EP_MAX_ENDPOINTS],
            ep_out_max_packet_size: [0; crate::EP_MAX_ENDPOINTS],
        }
    }

    #[inline(always)]
    pub fn dispatch_event(&mut self, event: InterruptEvent) {
        if matches!(event, InterruptEvent::Usb(crate::UsbInterface::Target, UsbEvent::BusReset)) {
            // send bus reset events to facedancer, but handle it locally for lower latency
            self.usb0.bus_reset();
            trace!("MD => UsbEvent::BusReset");

        } else if matches!(event, InterruptEvent::Usb(crate::UsbInterface::Target, UsbEvent::ReceiveControl(0))) {

        } else {
            debug!("\n\nMD => {:?}", event);
        }
        match self.queue.enqueue(event) {
            Ok(()) => (),
            Err(_) => {
                error!("Moondancer - event queue overflow");
                loop {
                    unsafe {
                        riscv::asm::nop();
                    }
                }
            }
        }
    }
}

// - usb0 interrupt handlers --------------------------------------------------

impl Moondancer {
    pub unsafe fn enable_usb_interrupts(&self) {
        interrupt::enable(pac::Interrupt::USB0);
        interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB0_EP_IN);
        interrupt::enable(pac::Interrupt::USB0_EP_OUT);

        // enable all usb events
        self.usb0.enable_interrupts();
    }

    pub unsafe fn disable_usb_interrupts(&self) {
        // disable all usb events
        self.usb0.disable_interrupts();

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
        #[derive(FromBytes, Unaligned)]
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

        match device_speed {
            Speed::High => {
                self.usb0.controller.full_speed_only.write(|w| w.full_speed_only().bit(false));
                self.usb0.controller.low_speed_only.write(|w| w.low_speed_only().bit(false));
            },
            Speed::Full => {
                self.usb0.controller.full_speed_only.write(|w| w.full_speed_only().bit(true));
                self.usb0.controller.low_speed_only.write(|w| w.low_speed_only().bit(false));
            },
            Speed::Low => {
                // FIXME still connects at full speed
                self.usb0.controller.full_speed_only.write(|w| w.full_speed_only().bit(false));
                self.usb0.controller.low_speed_only.write(|w| w.low_speed_only().bit(true));
            },
            _ => {
                log::warn!("Requested unsupported device speed, ignoring: {:?}", device_speed);
            }
        }

        // connect usb0 device and enable interrupts
        self.usb0.connect();
        unsafe { self.enable_usb_interrupts() };

        // wait for things to settle and get connection speed
        unsafe { riscv::asm::delay(5_000_000); }
        let speed: Speed = self.usb0.controller.speed.read().speed().bits().into();

        log::info!(
            "MD moondancer::connect(ep0_max_packet_size:{}, device_speed:{:?}, quirk_flags:{}) -> {:?}",
            args.ep0_max_packet_size, device_speed, args.quirk_flags, speed
        );

        Ok([].into_iter())
    }

    /// Terminate all existing communication and disconnects the USB interface.
    pub fn disconnect(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        // disable speed registers
        self.usb0.controller.full_speed_only.write(|w| w.full_speed_only().bit(false));
        self.usb0.controller.low_speed_only.write(|w| w.low_speed_only().bit(false));

        self.usb0.disconnect();

        // reset state
        self.quirk_flags = 0;
        self.ep_in_max_packet_size = [0; crate::EP_MAX_ENDPOINTS];
        self.ep_out_max_packet_size = [0; crate::EP_MAX_ENDPOINTS];

        debug!("MD moondancer::disconnect()");

        Ok([].into_iter())
    }

    /// Perform a USB bus reset.
    pub fn bus_reset(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        // we send the event to facedancer but the actual reset happens locally
        //self.usb0.bus_reset();

        trace!("MD moondancer::bus_reset()");

        Ok([].into_iter())
    }
}

// - verb implementations: status & control -----------------------------------

impl Moondancer {
    /// Read a control packet from SetupFIFOInterface.
    pub fn read_control(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let mut setup_packet_buffer = [0_u8; 8];
        self.usb0.read_control(&mut setup_packet_buffer);

        let setup_packet = SetupPacket::try_from(setup_packet_buffer)
            .map_err(|_| GreatError::IllegalByteSequence)?;

        debug!("MD moondancer::read_control() -> {:?}", setup_packet);

        Ok(SetupPacket::as_bytes(setup_packet).into_iter())
    }

    /// Set the device address.
    pub fn set_address(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
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
        #[derive(Debug, FromBytes, Unaligned)]
        struct ArgEndpoint {
            address: u8,
            max_packet_size: U16<LittleEndian>,
            transfer_type: u8,
        }

        log::info!("MD moondancer::configure_endpoints()");

        // while we have endpoint triplets to handle
        let mut byte_slice = arguments;
        while let Some((endpoint, next)) =
            zerocopy::LayoutVerified::<_, ArgEndpoint>::new_from_prefix(byte_slice)
        {
            let endpoint_number = (endpoint.address & 0x7f) as u8;

            log::info!(
                "  moondancer::configure_endpoint(0x{:x}) -> {} -> max_packet_size:{}",
                endpoint.address,
                endpoint_number,
                endpoint.max_packet_size
            );
            byte_slice = next;

            // endpoint zero is always the control endpoint, and can't be configured
            if endpoint_number == 0x00 {
                warn!("  ignoring request to reconfigure control endpoint address: 0x{:x}", endpoint.address);
                continue;
            }

            // ignore endpoint configurations we won't be able to handle
            if endpoint.max_packet_size.get() as usize > crate::EP_MAX_PACKET_SIZE {
                error!(
                    "  failed to configure endpoint address 0x{:x} with max packet size {} > {}",
                    endpoint.address,
                    endpoint.max_packet_size,
                    crate::EP_MAX_PACKET_SIZE,
                );
                return Err(GreatError::InvalidArgument);
            }

            // configure endpoint max packet sizes
            if Direction::from_endpoint_address(endpoint.address) == Direction::HostToDevice {
                self.ep_out_max_packet_size[endpoint_number as usize] = endpoint.max_packet_size.into();
            } else {
                self.ep_in_max_packet_size[endpoint_number as usize] = endpoint.max_packet_size.into();
            }

            // prime any OUT endpoints
            if Direction::from_endpoint_address(endpoint.address) == Direction::HostToDevice {
                log::info!(
                    "  priming HostToDevice (OUT) endpoint address: {}",
                    endpoint.address
                );
                self.usb0.ep_out_prime_receive(endpoint_number);
            }
        }

        let iter = [].into_iter();
        Ok(iter)
    }

    /// Stall the given USB endpoint.
    pub fn stall_endpoint(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_address: u8, // TODO consider using either endpoint_number or making _all_ api calls use address
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_address = args.endpoint_address;
        let endpoint_number = endpoint_address & 0x7f;

        // stall IN end
        self.usb0.stall_endpoint_in(endpoint_number);

        // stall OUT end
        self.usb0.stall_endpoint_out(endpoint_number);

        log::info!("MD moondancer::stall_endpoint({})", args.endpoint_address);

        Ok([].into_iter())
    }
}

// - verb implementations: data transfer --------------------------------------

impl Moondancer {
    pub fn read_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        // TODO bounds check / handle big responses
        let mut rx_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE] = [0; LIBGREAT_MAX_COMMAND_SIZE];
        let bytes_read = self.usb0.read(args.endpoint_number, &mut rx_buffer);

        // TODO should we automatically prime OUT receive instead of waiting for facedancer?
        //self.usb0.ep_out_prime_receive(args.endpoint_number);

        log::debug!(
            "MD moondancer::read_endpoint({}) -> bytes_read:{}",
            args.endpoint_number, bytes_read
        );

        Ok(rx_buffer.into_iter().take(bytes_read))
    }

    pub fn test_read_endpoint(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            payload_length: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        let payload_length: usize = u32::from(args.payload_length) as usize;

        debug!("MD moondancer::test_read_endpoint({})", payload_length);

        if payload_length > LIBGREAT_MAX_COMMAND_SIZE {
            debug!(
                "MD moondancer::test_read_endpoint error overflow: {}",
                payload_length
            );
            return Err(GreatError::NoBufferSpaceAvailable);
        }

        let mut rx_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE] = [0; LIBGREAT_MAX_COMMAND_SIZE];
        for (index, byte) in rx_buffer.iter_mut().enumerate() {
            *byte = (index % u8::MAX as usize) as u8;
        }

        Ok(rx_buffer.into_iter().take(payload_length))
    }

    pub fn ep_out_prime_receive(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
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

    pub fn write_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::LayoutVerified<B, u8>,
            blocking: zerocopy::LayoutVerified<B, u8>,
            payload: B,
        }
        let (endpoint_number, arguments) =
            zerocopy::LayoutVerified::new_unaligned_from_prefix(arguments)
            .ok_or(GreatError::InvalidArgument)?;
        let (blocking, payload) =
            zerocopy::LayoutVerified::new_unaligned_from_prefix(arguments)
                .ok_or(GreatError::InvalidArgument)?;
        let args = Args {
            endpoint_number,
            blocking,
            payload,
        };

        let endpoint_number: u8 = args.endpoint_number.read();
        let blocking = args.blocking.read() != 0;
        let payload_length = args.payload.len();
        let payload = args.payload.clone().iter();

        // TODO better handling for blocking
        if blocking {
            // set tx_ack_active flag
            // TODO a slighty safer approach would be nice
            unsafe {
                self.usb0.set_tx_ack_active();
            }
        }

        // TODO we can probably just use write_packets here
        let max_packet_size = self.ep_in_max_packet_size[endpoint_number as usize] as usize;
        if payload_length > max_packet_size {
            self.usb0
                .write_packets(endpoint_number, payload.copied(), max_packet_size);
        } else {
            self.usb0.write_ref(endpoint_number, payload);
        }

        // TODO better handling for blocking
        if blocking {
            // wait for the response packet to get sent
            // TODO a slightly safer approach would be nice
            loop {
                let active = unsafe { self.usb0.is_tx_ack_active() };
                if active == false {
                    break;
                }
            }
        }

        if payload_length > 0 {
            log::debug!(
                "MD moondancer::write_endpoint(endpoint_number:{}, blocking:{} payload.len:{} ({})) max_packet_size:{}",
                endpoint_number,
                blocking,
                payload_length,
                args.payload.iter().count(),
                max_packet_size,
            );
        }

        Ok([].into_iter())
    }

    pub fn test_write_endpoint(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::LayoutVerified<B, u8>,
            payload: B,
        }
        let (endpoint_number, payload) =
            zerocopy::LayoutVerified::new_unaligned_from_prefix(arguments)
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
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        let mut tx_buffer = [0_u8; LIBGREAT_MAX_COMMAND_SIZE];

        let clone = self.queue.clone();
        self.queue = Queue::new();

        let length = clone.len() * 3;
        let response = clone.iter().flat_map(|message| message.into_bytes());

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

        use crate::UsbInterface::{Aux, Control, Target};
        self.queue
            .enqueue(InterruptEvent::Usb(Target, UsbEvent::BusReset))
            .ok();
        self.queue
            .enqueue(InterruptEvent::Usb(Aux, UsbEvent::ReceiveControl(1)))
            .ok();
        self.queue
            .enqueue(InterruptEvent::Usb(Control, UsbEvent::ReceivePacket(2)))
            .ok();
        self.queue
            .enqueue(InterruptEvent::Usb(Target, UsbEvent::SendComplete(3)))
            .ok();

        self.get_interrupt_events(arguments)
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
pub static VERBS: [Verb; 14] = [
    // - device connection --
    Verb {
        id: 0x0,
        name: "connect\0",
        doc: "\0", //"Connect the target to the host. device_speed is 3:high, 2:full, 1:low\0",
        in_signature: "<HBH\0",
        in_param_names: "ep0_max_packet_size, device_speed, quirk_flags\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x1,
        name: "disconnect\0",
        doc: "\0", //"Disconnect the target port from the host.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x2,
        name: "bus_reset\0",
        doc: "\0", //"Cause the target device to handle a bus reset.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - status & control --
    Verb {
        id: 0x3,
        name: "read_control\0",
        doc: "\0", //"Read a setup packet from the control endpoint.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<8X\0",
        out_param_names: "setup_packet\0",
    },
    Verb {
        id: 0x4,
        name: "set_address\0",
        doc: "\0", //"Set the address of the target device.\nIf deferred is set this action won't complete until the setup phase ends.\0",
        in_signature: "<BB\0",
        in_param_names: "address, deferred\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x5,
        name: "configure_endpoints\0", // TODO s/prime_out_endpoint
        doc: "\0", //"Set up all of the non-control endpoints for the device.\0",
        in_signature: "<*(BHB)\0",
        in_param_names: "endpoint_descriptors\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x6,
        name: "stall_endpoint\0",
        doc: "\0", //"Stall the endpoint with the provided address.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_address\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - data transfer --
    Verb {
        id: 0x7,
        name: "read_endpoint\0",
        doc: "\0", //"Read a packet from an OUT endpoint.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "<*X\0",
        out_param_names: "read_data\0",
    },
    Verb {
        id: 0x8,
        name: "ep_out_prime_receive\0",
        doc: "\0", //"Prepare OUT endpoint to receive a single packet.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x9,
        name: "write_endpoint\0",
        doc: "\0", //"Write a packet to an IN endpoint.\0",
        in_signature: "<BB*X\0",
        in_param_names: "endpoint_number, blocking, payload\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    // - interrupts --
    Verb {
        id: 0xa,
        name: "get_interrupt_events\0",
        doc: "\0", //"Return the most recent driver messages.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<*(BBB)\0",
        out_param_names: "type, interface, endpoint\0",
    },
    // - tests --
    Verb {
        id: 0x27,
        name: "test_read_endpoint\0",
        doc: "\0", //"Return read_endpoint with payload_length of test data.\0",
        in_signature: "<I\0",
        in_param_names: "payload_length\0",
        out_signature: "<*X\0",
        out_param_names: "read_data\0",
    },
    Verb {
        id: 0x29,
        name: "test_write_endpoint\0",
        doc: "\0", //"Write a packet to an IN endpoint and return the length received.\0",
        in_signature: "<B*X\0",
        in_param_names: "endpoint_number, payload\0",
        out_signature: "<I\0",
        out_param_names: "payload_length\0",
    },
    Verb {
        id: 0x2a,
        name: "test_get_interrupt_events\0",
        doc: "\0", //"Return get_interrupt_events() with test data.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<*(BBB)\0",
        out_param_names: "type, interface, endpoint\0",
    },
];

// - dispatch -----------------------------------------------------------------

impl Moondancer {
    pub fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse> {
        match verb_number {
            0x0 => {
                // moondancer::connect
                let iter = self.connect(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x1 => {
                // moondancer::disconnect
                let iter = self.disconnect(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x2 => {
                // moondancer::bus_reset
                let iter = self.bus_reset(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x3 => {
                // moondancer::read_control
                let iter = self.read_control(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x4 => {
                // moondancer::set_address
                let iter = self.set_address(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x5 => {
                // moondancer::configure_endpoints
                let iter = self.configure_endpoints(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x6 => {
                // moondancer::stall_endpoint
                let iter = self.stall_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x7 => {
                // moondancer::read_endpoint
                let iter = self.read_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x8 => {
                // moondancer::ep_out_prime_receive
                let iter = self.ep_out_prime_receive(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x9 => {
                // moondancer::write_endpoint
                let iter = self.write_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0xa => {
                // moondancer::get_interrupt_events
                let iter = self.get_interrupt_events(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }

            // test APIs
            0x27 => {
                // moondancer::test_read_endpoint
                let iter = self.test_read_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x29 => {
                // moondancer::test_write_endpoint
                let iter = self.test_write_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x2a => {
                // moondancer::test_get_interrupt_events
                let iter = self.test_get_interrupt_events(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }

            verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
