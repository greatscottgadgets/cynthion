#![allow(dead_code, unused_imports, unused_variables)] // TODO
#![no_std]
#![no_main]

use moondancer::usb::vendor::{VendorRequest, VendorValue};
use moondancer::{hal, pac, Message};

use pac::csr::interrupt;

use smolusb::class;
use smolusb::control::{Direction, RequestType, SetupPacket};
use smolusb::device::{Speed, UsbDevice};
use smolusb::traits::{
    ControlRead, EndpointRead, EndpointWrite, EndpointWriteRef, UnsafeUsbDriverOperations,
    UsbDriverOperations,
};

use libgreat::gcp::{iter_to_response, GcpResponse, GCP_MAX_RESPONSE_LENGTH};
use libgreat::{GreatError, GreatResult};

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info, trace, warn};

use core::any::Any;
use core::{array, iter, slice};

// - global static state ------------------------------------------------------

static MESSAGE_QUEUE: Queue<Message, 128> = Queue::new();

#[inline(always)]
fn dispatch_message(message: Message) {
    match MESSAGE_QUEUE.enqueue(message) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - message queue overflow");
            //panic!("MachineExternal - message queue overflow");
            loop {
                unsafe {
                    riscv::asm::nop();
                }
            }
        }
    }
}

// - MachineExternal interrupt handler ----------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    use moondancer::UsbInterface::{Aux, Target};

    // peripherals
    let peripherals = unsafe { pac::Peripherals::steal() };
    let usb0 = unsafe { hal::Usb0::summon() };
    let usb1 = unsafe { hal::Usb1::summon() };

    let pending = interrupt::reg_pending();

    // - usb1 interrupts - "aux_phy" (host on r0.4) --

    // USB1 UsbBusReset
    if usb1.is_pending(pac::Interrupt::USB1) {
        usb1.clear_pending(pac::Interrupt::USB1);
        usb1.bus_reset();
        dispatch_message(Message::UsbBusReset(Aux));

    // USB1_EP_CONTROL UsbReceiveSetupPacket
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        let endpoint = usb1.ep_control.epno.read().bits() as u8;
        let mut setup_packet_buffer = [0_u8; 8];
        usb1.read_control(&mut setup_packet_buffer);
        usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        let message = match SetupPacket::try_from(setup_packet_buffer) {
            Ok(setup_packet) => Message::UsbReceiveSetupPacket(Aux, endpoint, setup_packet),
            Err(e) => Message::ErrorMessage("USB1_EP_CONTROL failed to read setup packet"),
        };
        dispatch_message(message);

    // USB1_EP_OUT UsbReceiveData
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        let endpoint = usb1.ep_out.data_ep.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);
        dispatch_message(Message::UsbReceivePacket(Aux, endpoint, 0));

    // USB1_EP_IN UsbTransferComplete
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        let endpoint = usb1.ep_in.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_IN);

        // TODO something a little safer would be nice
        unsafe {
            usb1.clear_tx_ack_active();
        }

        dispatch_message(Message::UsbSendComplete(Aux, endpoint));

    // - usb0 interrupts - "target_phy" --

    // USB0 UsbBusReset
    } else if usb0.is_pending(pac::Interrupt::USB0) {
        usb0.clear_pending(pac::Interrupt::USB0);
        dispatch_message(Message::UsbBusReset(Target));

    // USB0_EP_CONTROL UsbReceiveSetupPacket
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        let endpoint = usb0.ep_control.epno.read().bits() as u8;
        let mut setup_packet_buffer = [0_u8; 8];
        usb0.read_control(&mut setup_packet_buffer);
        usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
        let message = match SetupPacket::try_from(setup_packet_buffer) {
            Ok(setup_packet) => Message::UsbReceiveSetupPacket(Target, endpoint, setup_packet),
            Err(e) => Message::ErrorMessage("USB0_EP_CONTROL failed to read setup packet"),
        };
        dispatch_message(message);

    // USB0_EP_OUT UsbReceiveData
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        let endpoint = usb0.ep_out.data_ep.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);
        dispatch_message(Message::UsbReceivePacket(Target, endpoint, 0));

    // USB0_EP_IN UsbTransferComplete
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        let endpoint = usb0.ep_in.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_IN);

        // TODO something a little bit safer would be nice
        unsafe {
            usb0.clear_tx_ack_active();
        }

        dispatch_message(Message::UsbSendComplete(Target, endpoint));

    // - Unknown Interrupt --
    } else {
        dispatch_message(Message::HandleUnknownInterrupt(pending));
    }
}

// - main entry point ---------------------------------------------------------

#[cfg(feature = "vexriscv")]
#[riscv_rt::pre_init]
unsafe fn pre_main() {
    pac::cpu::vexriscv::flush_icache();
    #[cfg(feature = "vexriscv_dcache")]
    pac::cpu::vexriscv::flush_dcache();
}

#[riscv_rt::entry]
fn main() -> ! {
    // initialize firmware
    let mut firmware = Firmware::new(pac::Peripherals::take().unwrap());
    match firmware.initialize() {
        Ok(()) => (),
        Err(e) => {
            error!("Firmware panicked during initialization: {}", e);
            //panic!("Firmware panicked during initialization: {}", e)
        }
    }

    // enter main loop
    match firmware.main_loop() {
        Ok(()) => {
            error!("Firmware exited unexpectedly in main loop");
            //panic!("Firmware exited unexpectedly in main loop")
        }
        Err(e) => {
            error!("Firmware panicked in main loop: {}", e);
            //panic!("Firmware panicked in main loop: {}", e)
        }
    }

    loop {
        unsafe {
            riscv::asm::nop();
        }
    }
}

// - Firmware -----------------------------------------------------------------

struct Firmware<'a> {
    // peripherals
    leds: pac::LEDS,
    usb1: UsbDevice<'a, hal::Usb1>,

    // state
    gcp_response: Option<GcpResponse<'a>>,
    gcp_response_last_error: Option<GreatError>,

    // classes
    core: libgreat::gcp::class_core::Core,
    moondancer: moondancer::gcp::moondancer::Moondancer,
}

impl<'a> Firmware<'a> {
    fn new(peripherals: pac::Peripherals) -> Self {
        // initialize logging
        moondancer::log::init(hal::Serial::new(peripherals.UART));
        info!("Logging initialized");

        // usb1: aux (host on r0.4)
        let mut usb1 = UsbDevice::new(
            hal::Usb1::new(
                peripherals.USB1,
                peripherals.USB1_EP_CONTROL,
                peripherals.USB1_EP_IN,
                peripherals.USB1_EP_OUT,
            ),
            &moondancer::usb::DEVICE_DESCRIPTOR,
            &moondancer::usb::CONFIGURATION_DESCRIPTOR_0,
            &moondancer::usb::USB_STRING_DESCRIPTOR_0,
            &moondancer::usb::USB_STRING_DESCRIPTORS,
        );
        usb1.device_qualifier_descriptor = Some(&moondancer::usb::DEVICE_QUALIFIER_DESCRIPTOR);
        usb1.other_speed_configuration_descriptor =
            Some(moondancer::usb::OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0);

        // usb0: target
        let usb0 = hal::Usb0::new(
            peripherals.USB0,
            peripherals.USB0_EP_CONTROL,
            peripherals.USB0_EP_IN,
            peripherals.USB0_EP_OUT,
        );

        // initialize class registry
        static CLASSES: [libgreat::gcp::Class; 4] = [
            libgreat::gcp::class_core::CLASS,
            moondancer::gcp::firmware::CLASS,
            moondancer::gcp::selftest::CLASS,
            moondancer::gcp::moondancer::CLASS,
        ];
        let classes = libgreat::gcp::Classes(&CLASSES);

        // initialize classes
        let core = libgreat::gcp::class_core::Core::new(classes, moondancer::BOARD_INFORMATION);
        let moondancer = moondancer::gcp::moondancer::Moondancer::new(usb0);

        Self {
            leds: peripherals.LEDS,
            usb1,
            gcp_response: None,
            gcp_response_last_error: None,
            core,
            moondancer,
        }
    }

    fn initialize(&mut self) -> GreatResult<()> {
        // leds: starting up
        self.leds
            .output
            .write(|w| unsafe { w.output().bits(1 << 2) });

        // connect usb1
        let speed = self.usb1.connect();
        info!("Connected usb1 device: {:?}", speed);

        // enable interrupts
        unsafe {
            // set mstatus register: interrupt enable
            riscv::interrupt::enable();

            // set mie register: machine external interrupts enable
            riscv::register::mie::set_mext();

            // write csr: enable usb1 interrupts and events
            self.enable_usb1_interrupts();
        }

        Ok(())
    }

    #[inline(always)]
    fn main_loop(&'a mut self) -> GreatResult<()> {
        let mut rx_buffer: [u8; moondancer::EP_MAX_PACKET_SIZE] =
            [0; moondancer::EP_MAX_PACKET_SIZE];
        let mut max_queue_length = 0;
        let mut queue_length = 0;

        info!("Peripherals initialized, entering main loop");

        let mut counter: usize = 1;

        loop {
            // leds: main loop is responsive, interrupts are firing
            self.leds
                .output
                .write(|w| unsafe { w.output().bits((counter % 256) as u8) });

            if queue_length > max_queue_length {
                max_queue_length = queue_length;
                debug!("max_queue_length: {}", max_queue_length);
            }
            queue_length = 0;

            while let Some(message) = MESSAGE_QUEUE.dequeue() {

                //debug!("MachineExternal: {:?}", message);
                counter += 1;

                // leds: message loop is active
                self.leds
                    .output
                    .write(|w| unsafe { w.output().bits(1 << 0) });

                use moondancer::{
                    Message::*,
                    UsbInterface::{Aux, Target},
                };

                queue_length += 1;

                match message {
                    // - usb1 message handlers --

                    // Usb1 received USB bus reset
                    UsbBusReset(Aux) => {
                        // handled in MachineExternal
                    }

                    // Usb1 received setup packet
                    UsbReceiveSetupPacket(Aux, endpoint_number, packet) => {
                        self.handle_receive_setup_packet(endpoint_number, packet)?;
                    }

                    // Usb1 received data on control endpoint
                    UsbReceivePacket(Aux, 0, _) => {
                        let bytes_read = self.usb1.hal_driver.read(0, &mut rx_buffer);
                        self.handle_receive_control_data(bytes_read, rx_buffer)?;
                        self.usb1.hal_driver.ep_out_prime_receive(0);
                    }

                    // Usb1 received data on endpoint - shouldn't ever be called
                    UsbReceivePacket(Aux, endpoint_number, _) => {
                        let bytes_read = self.usb1.hal_driver.read(endpoint_number, &mut rx_buffer);
                        self.handle_receive_data(endpoint_number, bytes_read, rx_buffer)?;
                        self.usb1.hal_driver.ep_out_prime_receive(endpoint_number);
                    }

                    // Usb1 transfer complete
                    UsbSendComplete(Aux, endpoint_number) => {
                        self.handle_transfer_complete(endpoint_number)?;
                    }

                    // - usb0 message handlers --

                    // Usb0 received USB bus reset
                    UsbBusReset(Target) => {
                        warn!("USB0 UsbBusReset");
                        self.moondancer.handle_bus_reset()?;
                    }

                    // Usb0 received setup packet
                    UsbReceiveSetupPacket(Target, endpoint_number, packet) => {
                        warn!("USB0_EP_CONTROL UsbReceiveSetupPacket({})", endpoint_number);
                        self.moondancer.handle_receive_setup_packet(endpoint_number, packet)?;
                    }

                    // Usb0 received data on control endpoint
                    UsbReceivePacket(Target, 0, _) => {
                        warn!("USB0_EP_OUT UsbReceivePacket(control)");
                        let bytes_read = self.moondancer.usb0.read(0, &mut rx_buffer);
                        self.moondancer
                            .handle_receive_control_data(bytes_read, rx_buffer)?;
                        // TODO maybe we want to do this _after_ facedancer is done
                        self.moondancer.usb0.ep_out_prime_receive(0);
                    }

                    // Usb0 received data on endpoint
                    UsbReceivePacket(Target, endpoint_number, _) => {
                        warn!("USB0_EP_OUT UsbReceivePacket({})", endpoint_number);
                        let bytes_read = self.moondancer.usb0.read(endpoint_number, &mut rx_buffer);
                        self.moondancer
                            .handle_receive_data(endpoint_number, bytes_read, rx_buffer)?;
                        // TODO maybe we want to do this _after_ facedancer is done
                        self.moondancer.usb0.ep_out_prime_receive(endpoint_number);
                    }

                    // Usb0 transfer complete
                    UsbSendComplete(Target, endpoint_number) => {
                        warn!("USB0_EP_IN UsbTransferComplete({})", endpoint_number);
                        self.moondancer.handle_transfer_complete(endpoint_number)?;
                    }

                    // Error Message
                    ErrorMessage(message) => {
                        error!("MachineExternal Error - {}", message);
                    }

                    // Unhandled message
                    _ => {
                        error!("Unhandled message: {:?}", message);
                    }
                }
            }
        }

        #[allow(unreachable_code)] // TODO
        Ok(())
    }
}

// - usb1 interrupt handlers ----------------------------------------------

impl<'a> Firmware<'a> {
    unsafe fn enable_usb1_interrupts(&self) {
        interrupt::enable(pac::Interrupt::USB1);
        interrupt::enable(pac::Interrupt::USB1_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB1_EP_IN);
        interrupt::enable(pac::Interrupt::USB1_EP_OUT);

        // enable all usb events
        self.usb1.hal_driver.enable_interrupts();
    }

    fn handle_receive_setup_packet(&mut self, endpoint_number: u8, setup_packet: SetupPacket) -> GreatResult<()> {
        let request_type = setup_packet.request_type();
        let vendor_request = VendorRequest::from(setup_packet.request);

        trace!(
            "Control packet: {:?} {:?}",
            setup_packet.direction(),
            setup_packet
        );

        match (&request_type, &vendor_request) {
            (RequestType::Vendor, VendorRequest::UsbCommandRequest) => {
                self.handle_vendor_request(&setup_packet)?;
            }
            (RequestType::Vendor, VendorRequest::Unknown(vendor_request)) => {
                error!("GCP Unknown vendor request '{}'", vendor_request);
                // TODO how to handle? should it be handled?
            }
            (RequestType::Vendor, vendor_request) => {
                // TODO this is from one of the legacy boards which we
                // need to support to get `greatfet info` to finish
                // enumerating through the supported devices.
                //
                // see: host/greatfet/boards/legacy.py

                // The greatfet board scan code expects the endpoint
                // to be stalled if this is not a legacy device.
                self.usb1.hal_driver.stall_endpoint_in(0);

                warn!("GCP Legacy vendor request '{:?}'", vendor_request);

                // enable these if you want to pretend to be a legacy greatfet device :-)
                /*match vendor_request {
                    VendorRequest::LegacyReadBoardId => {
                        self.usb1.hal_driver.write(0, [0].into_iter());
                    }
                    VendorRequest::LegacyReadVersionString => {
                        let version_string = moondancer::BOARD_INFORMATION.version_string.as_bytes();
                        self.usb1.hal_driver.write(0, version_string.into_iter().copied());
                    }
                    VendorRequest::LegacyReadPartId => {
                        let part_id = moondancer::BOARD_INFORMATION.part_id;
                        self.usb1.hal_driver.write(0, part_id.into_iter());
                    }
                    _ => {
                        error!("TODO");
                    }
                }*/
            }
            _ => match self.usb1.handle_setup_request(endpoint_number, &setup_packet) {
                Ok(()) => (),
                Err(e) => {
                    error!("Failed to handle setup request: {:?}: {:?}", e, setup_packet);
                    return Err(GreatError::BadMessage);
                }
            },
        }
        Ok(())
    }

    /// Usb1: gcp vendor request handler
    fn handle_vendor_request(&mut self, setup_packet: &SetupPacket) -> GreatResult<()> {
        let direction = setup_packet.direction();
        let request = VendorRequest::from(setup_packet.request);
        let value = VendorValue::from(setup_packet.value);
        let length = setup_packet.length as usize;

        trace!(
            "GCP vendor_request: {:?} dir:{:?} value:{:?} length:{} index:{}",
            request,
            direction,
            value,
            length,
            setup_packet.index
        );

        match (&direction, &request, &value) {
            // host is starting a new command sequence
            (Direction::HostToDevice, VendorRequest::UsbCommandRequest, VendorValue::Execute) => {
                self.usb1.hal_driver.ack_status_stage(setup_packet);
            }

            // host is ready to receive a response
            (Direction::DeviceToHost, VendorRequest::UsbCommandRequest, VendorValue::Execute) => {
                self.dispatch_gcp_response(setup_packet)?;
            }

            // host would like to abort the current command sequence
            (Direction::DeviceToHost, VendorRequest::UsbCommandRequest, VendorValue::Cancel) => {
                self.dispatch_gcp_abort(setup_packet)?;
            }

            _ => {
                error!(
                    "GCP stall: unknown vendor request and/or value: {:?} {:?} {:?}",
                    direction, request, value
                );
                self.usb1.hal_driver.stall_endpoint_address(0, true);
            }
        }

        Ok(())
    }

    fn handle_receive_control_data(
        &mut self,
        bytes_read: usize,
        buffer: [u8; moondancer::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        trace!("Received {} bytes on usb1 control endpoint", bytes_read,);

        if bytes_read >= 8 {
            // it's gcp request data, dispatch it
            self.dispatch_gcp_request(&buffer[0..bytes_read])?;
        } else {
            // it's an ack for the last gcp response we sent, ignore it
        }

        Ok(())
    }

    /// This shouldn't ever be called
    fn handle_receive_data(
        &mut self,
        endpoint: u8,
        bytes_read: usize,
        buffer: [u8; moondancer::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        warn!(
            "Usb1 received {} bytes on endpoint: {}",
            endpoint, bytes_read,
        );
        Ok(())
    }

    /// TODO we should probably take this into account for state handling
    pub fn handle_transfer_complete(&mut self, endpoint: u8) -> GreatResult<()> {
        Ok(())
    }
}

// - gcp command dispatch -----------------------------------------------------

impl<'a> Firmware<'a> {
    fn dispatch_gcp_request(&mut self, command_buffer: &[u8]) -> GreatResult<()> {
        // parse command
        let (class_id, verb_number, arguments) = match libgreat::gcp::Command::parse(command_buffer)
        {
            Some(command) => (command.class_id(), command.verb_number(), command.arguments),
            None => {
                // TODO some kind of error handling
                error!("Failed to parse GCP command");
                return Ok(());
            }
        };

        //debug!("GCP dispatch request {:?}.{}", class_id, verb_number);

        // dispatch command
        let response_buffer: [u8; GCP_MAX_RESPONSE_LENGTH] = [0; GCP_MAX_RESPONSE_LENGTH];
        let response = match class_id {
            // class: core
            libgreat::gcp::ClassId::core => {
                self.core.dispatch(verb_number, arguments, response_buffer)
            }
            // class: firmware
            libgreat::gcp::ClassId::firmware => {
                moondancer::gcp::firmware::dispatch(verb_number, arguments, response_buffer)
            }
            // class: selftest
            libgreat::gcp::ClassId::selftest => {
                moondancer::gcp::selftest::dispatch(verb_number, arguments, response_buffer)
            }
            // class: moondancer
            libgreat::gcp::ClassId::moondancer => {
                self.moondancer
                    .dispatch(verb_number, arguments, response_buffer)
            }
            // class: unsupported
            _ => {
                error!("GCP dispatch request error: Class id '{:?}' not found", class_id);
                Err(GreatError::InvalidArgument)
            },
        };

        // queue response
        match response {
            Ok(response) => {
                // TODO we really need a better way to get this to the vendor request
                // NEXT so what's happening with greatfet info is that we queue
                //      the response but the host errors out before we get the
                //      vendor_request telling us we can send it ???
                //debug!("GCP queueing response");
                self.gcp_response = Some(response);
                self.gcp_response_last_error = None;
            }
            Err(e) => {
                error!("GCP error: failed to dispatch command {}", e);
                self.gcp_response = None;
                self.gcp_response_last_error = Some(e);
                // TODO set a proper errno
                /*self.usb1
                    .hal_driver
                    .write(0, [0xde, 0xad, 0xde, 0xad].into_iter());*/
                self.usb1.hal_driver.stall_endpoint_in(0);
            }
        }

        Ok(())
    }

    fn dispatch_gcp_response(&mut self, setup_packet: &SetupPacket) -> GreatResult<()> {
        // do we have a response ready?
        if let Some(response) = &mut self.gcp_response {
            // send it
            // debug!("GCP dispatch response: {} bytes", response.len());
            // TODO handle long writes -> setup_packet.length as usize is 4096
            self.usb1
                .hal_driver
                .write(0, response.take(moondancer::EP_MAX_PACKET_SIZE));
            self.gcp_response = None;

        } else if let Some(error) = self.gcp_response_last_error {
            warn!("dispatch_gcp_response error result: {:?}", error);
            self.usb1
                    .hal_driver
                    .write(0, (error as u32).to_le_bytes().into_iter());
            self.gcp_response_last_error = None;

        } else {
            // TODO figure out what to do if we don't have a response or error
            error!("GCP stall: gcp response requested but no response or error queued");
            self.usb1.hal_driver.stall_endpoint_in(0);
        }

        Ok(())
    }

    fn dispatch_gcp_abort(&mut self, setup_packet: &SetupPacket) -> GreatResult<()> {
        debug!("GCP dispatch abort");

        // cancel any queued response
        self.gcp_response = None;
        self.gcp_response_last_error = None;

        // TODO figure out if the host is expecting a response
        /*self.usb1
            .hal_driver
            .write(0, [].into_iter());*/

        Ok(())
    }
}
