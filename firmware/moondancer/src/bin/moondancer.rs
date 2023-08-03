#![allow(dead_code, unused_imports, unused_mut, unused_variables)] // TODO
#![no_std]
#![no_main]

use core::any::Any;
use core::{array, iter, slice};

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info, trace, warn};

use smolusb::class;
use smolusb::control::ControlEvent;
use smolusb::device::{Speed, UsbDevice};
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, RequestType, SetupPacket};
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
    WriteRefEndpoint,
};

use libgreat::gcp::{iter_to_response, GreatResponse, LIBGREAT_MAX_COMMAND_SIZE};
use libgreat::{GreatError, GreatResult};

use moondancer::event::InterruptEvent;
use moondancer::usb::vendor::{VendorRequest, VendorValue};
use moondancer::{hal, pac};

use pac::csr::interrupt;

// - MachineExternal interrupt handler ----------------------------------------

static EVENT_QUEUE: Queue<InterruptEvent, 128> = Queue::new();

#[inline(always)]
fn dispatch_event(event: InterruptEvent) {
    match EVENT_QUEUE.enqueue(event) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - event queue overflow");
            loop {
                unsafe {
                    riscv::asm::nop();
                }
            }
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    match moondancer::util::get_usb_interrupt_event() {
        InterruptEvent::UnhandledInterrupt(pending) => {
            dispatch_event(InterruptEvent::UnknownInterrupt(pending));
        }
        event => {
            dispatch_event(event);
        }
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
            panic!("Firmware panicked during initialization: {}", e)
        }
    }

    // enter main loop
    match firmware.main_loop() {
        Ok(()) => {
            panic!("Firmware exited unexpectedly in main loop")
        }
        Err(e) => {
            panic!("Firmware panicked in main loop: {}", e)
        }
    }
}

// - Firmware -----------------------------------------------------------------

struct Firmware<'a> {
    // peripherals
    leds: pac::LEDS,
    usb1: UsbDevice<'a, hal::Usb1, { libgreat::gcp::LIBGREAT_MAX_COMMAND_SIZE }>,

    // state
    libgreat_response: Option<GreatResponse<'a>>,
    libgreat_response_last_error: Option<GreatError>,

    // classes
    core: libgreat::gcp::class_core::Core,
    moondancer: moondancer::gcp::moondancer::Moondancer,
}

// - lifecycle ----------------------------------------------------------------

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
            moondancer::usb::DEVICE_DESCRIPTOR,
            moondancer::usb::CONFIGURATION_DESCRIPTOR_0,
            moondancer::usb::USB_STRING_DESCRIPTOR_0,
            moondancer::usb::USB_STRING_DESCRIPTORS,
        );
        usb1.set_device_qualifier_descriptor(moondancer::usb::DEVICE_QUALIFIER_DESCRIPTOR);
        usb1.set_other_speed_configuration_descriptor(
            moondancer::usb::OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0,
        );

        // usb0: target
        let usb0 = hal::Usb0::new(
            peripherals.USB0,
            peripherals.USB0_EP_CONTROL,
            peripherals.USB0_EP_IN,
            peripherals.USB0_EP_OUT,
        );

        // initialize libgreat class registry
        static CLASSES: [libgreat::gcp::Class; 4] = [
            libgreat::gcp::class_core::CLASS,
            moondancer::gcp::firmware::CLASS,
            moondancer::gcp::selftest::CLASS,
            moondancer::gcp::moondancer::CLASS,
        ];
        let classes = libgreat::gcp::Classes(&CLASSES);

        // initialize libgreat classes
        let core = libgreat::gcp::class_core::Core::new(classes, moondancer::BOARD_INFORMATION);
        let moondancer = moondancer::gcp::moondancer::Moondancer::new(usb0);

        Self {
            leds: peripherals.LEDS,
            usb1,
            libgreat_response: None,
            libgreat_response_last_error: None,
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
            interrupt::enable(pac::Interrupt::USB1);
            interrupt::enable(pac::Interrupt::USB1_EP_CONTROL);
            interrupt::enable(pac::Interrupt::USB1_EP_IN);
            interrupt::enable(pac::Interrupt::USB1_EP_OUT);

            // enable all usb events
            self.usb1.hal_driver.enable_interrupts();
        }

        Ok(())
    }
}

// - main loop ----------------------------------------------------------------

impl<'a> Firmware<'a> {
    #[inline(always)]
    fn main_loop(&'a mut self) -> GreatResult<()> {
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

            while let Some(event) = EVENT_QUEUE.dequeue() {
                counter += 1;
                queue_length += 1;

                // leds: event loop is active
                self.leds
                    .output
                    .write(|w| unsafe { w.output().bits(1 << 0) });

                use moondancer::{
                    event::InterruptEvent::*,
                    UsbInterface::{Aux, Target},
                };
                use smolusb::event::UsbEvent::*;

                match event {
                    // - misc event handlers --
                    ErrorMessage(message) => {
                        error!("MachineExternal Error - {}", message);
                    }

                    // - usb1 event handlers --

                    // Usb1 received a control event
                    Usb(Aux, event @ BusReset)
                    | Usb(Aux, event @ ReceiveSetupPacket(0))
                    | Usb(Aux, event @ ReceivePacket(0))
                    | Usb(Aux, event @ SendComplete(0)) => {
                        trace!("Usb(Aux, {:?})", event);
                        // TODO better error?
                        match self
                            .usb1
                            .dispatch_control(event)
                            .map_err(|_| GreatError::IoError)?
                        {
                            Some(control_event) => {
                                // handle any events control couldn't
                                self.handle_control_event(control_event)?;
                            }
                            None => {
                                // control event was handled by UsbDevice
                            }
                        }
                    }

                    // - usb0 event handlers --

                    // enqueue moondancer events
                    Usb(Target, event) => match self
                        .moondancer
                        .queue
                        .enqueue(InterruptEvent::from_smolusb_event(Target, event))
                    {
                        Ok(()) => (),
                        Err(_) => {
                            error!("Moondancer - event queue overflow");
                            loop {
                                unsafe {
                                    riscv::asm::nop();
                                }
                            }
                        }
                    },

                    // Unhandled event
                    _ => {
                        error!("Unhandled event: {:?}", event);
                    }
                }
            }
        }

        #[allow(unreachable_code)] // TODO
        Ok(())
    }
}

// - usb1 control handler -----------------------------------------------------

impl<'a> Firmware<'a> {
    /// Handle any control packets that weren't handled by UsbDevice
    fn handle_control_event(
        &mut self,
        control_event: ControlEvent<'a, { libgreat::gcp::LIBGREAT_MAX_COMMAND_SIZE }>,
    ) -> GreatResult<()> {
        let ControlEvent {
            setup_packet,
            data,
            bytes_read,
            ..
        } = control_event;
        let data = &data[..bytes_read];
        let direction = setup_packet.direction();
        let request_type = setup_packet.request_type();
        let vendor_request = VendorRequest::from(setup_packet.request);
        let vendor_value = VendorValue::from(setup_packet.value);

        trace!(
            "handle_control_event direction:{:?} packet:{:?} request_type:{:?} vendor_request:{:?} vendor_value:{:?} data:{:?}",
            direction,
            setup_packet,
            request_type,
            vendor_request,
            vendor_value,
            data,
        );

        match (&request_type, &vendor_request) {
            (RequestType::Vendor, VendorRequest::UsbCommandRequest) => {
                match (&vendor_value, &direction) {
                    // host is starting a new command sequence
                    (VendorValue::Execute, Direction::HostToDevice) => {
                        trace!("  GOT COMMAND data:{:?}", data);
                        self.dispatch_libgreat_request(data)?;
                    }

                    // host is ready to receive a response
                    (VendorValue::Execute, Direction::DeviceToHost) => {
                        trace!("  GOT RESPONSE REQUEST");
                        self.dispatch_libgreat_response(&setup_packet)?;
                    }

                    // host would like to abort the current command sequence
                    (VendorValue::Cancel, Direction::DeviceToHost) => {
                        debug!("  GOT ABORT");
                        self.dispatch_libgreat_abort(&setup_packet)?;
                    }

                    _ => {
                        error!(
                            "handle_control stall: unknown vendor request and/or value direction{:?} vendor_request{:?} vendor_value:{:?}",
                            direction, vendor_request, vendor_value
                        );
                        self.usb1.hal_driver.stall_endpoint_address(0, true);
                    }
                }
            }
            (RequestType::Vendor, VendorRequest::Unknown(vendor_request)) => {
                error!(
                    "handle_control_event Unknown vendor request '{}'",
                    vendor_request
                );
                return Err(GreatError::BadMessage);
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

                warn!(
                    "handle_control_event Legacy libgreat vendor request '{:?}'",
                    vendor_request
                );

                // enable these if you want to pretend to be a legacy greatfet device :-)
                /*match vendor_request {
                    VendorRequest::LegacyReadBoardId => {
                        self.usb1.hal_driver.write(0, [0].into_iter());
                    }
                    VendorRequest::LegacyReadVersionString => {
                        let version_string =
                            moondancer::BOARD_INFORMATION.version_string.as_bytes();
                        self.usb1
                            .hal_driver
                            .write(0, version_string.into_iter().copied());
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
            _ => {
                error!(
                    "handle_control_event Unknown control packet '{:?}'",
                    setup_packet
                );
                return Err(GreatError::BadMessage);
            }
        }

        Ok(())
    }
}

// - libgreat command dispatch ------------------------------------------------

impl<'a> Firmware<'a> {
    fn dispatch_libgreat_request(&mut self, command_buffer: &[u8]) -> GreatResult<()> {
        // parse command
        let (class_id, verb_number, arguments) = match libgreat::gcp::Command::parse(command_buffer)
        {
            Some(command) => (command.class_id(), command.verb_number(), command.arguments),
            None => {
                error!("dispatch_libgreat_request failed to parse libgreat command");
                return Err(GreatError::BadMessage);
            }
        };

        trace!("dispatch_libgreat_request {:?}.{}", class_id, verb_number);

        // dispatch command
        let response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE] = [0; LIBGREAT_MAX_COMMAND_SIZE];
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
                error!(
                    "dispatch_libgreat_request error: Class id '{:?}' not found",
                    class_id
                );
                Err(GreatError::InvalidArgument)
            }
        };

        // queue response
        match response {
            Ok(response) => {
                self.libgreat_response = Some(response);
                self.libgreat_response_last_error = None;
            }
            Err(e) => {
                error!(
                    "dispatch_libgreat_request error: failed to dispatch command {}",
                    e
                );
                self.libgreat_response = None;
                self.libgreat_response_last_error = Some(e);
                // TODO set a proper errno
                /*self.usb1
                .hal_driver
                .write(0, [0xde, 0xad, 0xde, 0xad].into_iter());*/
                self.usb1.hal_driver.stall_endpoint_in(0);
            }
        }

        Ok(())
    }

    fn dispatch_libgreat_response(&mut self, setup_packet: &SetupPacket) -> GreatResult<()> {
        // do we have a response ready?
        if let Some(response) = &mut self.libgreat_response {
            let bytes_to_send = response.len();

            if bytes_to_send > 60 {
                log::debug!("dispatch_libgreat_response -> {} bytes", response.len());
            }

            self.usb1.hal_driver.write_packets(0, response, 64);

            self.libgreat_response = None;
        } else if let Some(error) = self.libgreat_response_last_error {
            warn!("dispatch_libgreat_response error result: {:?}", error);
            self.usb1
                .hal_driver
                .write(0, (error as u32).to_le_bytes().into_iter());
            self.libgreat_response_last_error = None;
        } else {
            // TODO figure out what to do if we don't have a response or error
            error!("dispatch_libgreat_response stall: libgreat response requested but no response or error queued");
            self.usb1.hal_driver.stall_endpoint_in(0);
        }

        Ok(())
    }

    fn dispatch_libgreat_abort(&mut self, setup_packet: &SetupPacket) -> GreatResult<()> {
        debug!("dispatch_libgreat_response abort");

        // cancel any queued response
        self.libgreat_response = None;
        self.libgreat_response_last_error = None;

        // TODO figure out if the host is expecting a response
        /*self.usb1
        .hal_driver
        .write(0, [].into_iter());*/

        Ok(())
    }
}
