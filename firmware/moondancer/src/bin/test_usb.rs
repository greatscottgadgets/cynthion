#![allow(dead_code, unused_imports, unused_mut, unused_variables)] // TODO
#![no_std]
#![no_main]

use core::any::Any;
use core::{array, iter, slice};

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info, trace, warn};

use libgreat::gcp::{iter_to_response, GreatResponse, LIBGREAT_MAX_COMMAND_SIZE};
use libgreat::{GreatError, GreatResult};

use smolusb::class;
use smolusb::control::ControlEvent;
use smolusb::device::{Speed, UsbDevice};
use smolusb::setup::{Direction, RequestType, SetupPacket};
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
    WriteRefEndpoint,
};

use moondancer::event::InterruptEvent;
use moondancer::usb::vendor::{VendorRequest, VendorValue};
use moondancer::{hal, pac};

use pac::csr::interrupt;

const BULK_OUT_ENDPOINT_NUMBER: u8 = 0x02;

// - MachineExternal interrupt handler ----------------------------------------

static EVENT_QUEUE: Queue<InterruptEvent, 128> = Queue::new();

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
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

#[derive(Debug, PartialEq)]
enum State {
    None,
    NewCommandSequence,
    SendResponse,
    Error,
    Abort,
}

struct Firmware<'a> {
    // peripherals
    leds: pac::LEDS,
    usb1: UsbDevice<'a, hal::Usb1, { libgreat::gcp::LIBGREAT_MAX_COMMAND_SIZE }>,
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
            DEVICE_DESCRIPTOR,
            CONFIGURATION_DESCRIPTOR_0,
            USB_STRING_DESCRIPTOR_0,
            USB_STRING_DESCRIPTORS,
        );
        usb1.set_device_qualifier_descriptor(DEVICE_QUALIFIER_DESCRIPTOR);
        usb1.set_other_speed_configuration_descriptor(OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0);

        Self {
            leds: peripherals.LEDS,
            usb1,
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

        // prime our bulk OUT endpoint
        self.usb1
            .hal_driver
            .ep_out_prime_receive(BULK_OUT_ENDPOINT_NUMBER);

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
                    | Usb(Aux, event @ ReceiveControl(0))
                    | Usb(Aux, event @ ReceivePacket(0))
                    | Usb(Aux, event @ SendComplete(0)) => {
                        debug!("\n\nUsb(Aux, {:?})", event);
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

                    // USB1_EP_OUT 2 - Usb1 received data on command endpoint
                    Usb(Aux, event @ ReceivePacket(BULK_OUT_ENDPOINT_NUMBER)) => {
                        debug!("\n\nUsb(Aux, {:?})", event);
                        let bytes_read = self
                            .usb1
                            .hal_driver
                            .read(BULK_OUT_ENDPOINT_NUMBER, &mut rx_buffer);
                        self.handle_receive_command_packet(bytes_read, rx_buffer)?;
                        self.usb1
                            .hal_driver
                            .ep_out_prime_receive(BULK_OUT_ENDPOINT_NUMBER);
                    }

                    // USB1_EP_OUT n - Usb1 received data on endpoint - shouldn't ever be called
                    Usb(Aux, event @ ReceivePacket(endpoint_number)) => {
                        debug!("\n\nUsb(Aux, {:?})", event);
                        let bytes_read = self.usb1.hal_driver.read(endpoint_number, &mut rx_buffer);
                        self.handle_receive_packet(endpoint_number, bytes_read, rx_buffer)?;
                        self.usb1.hal_driver.ep_out_prime_receive(endpoint_number);
                    }

                    // USB1_EP_IN n Usb1 transfer complete
                    Usb(Aux, event @ SendComplete(endpoint_number)) => {
                        debug!("\n\nUsb(Aux, {:?})", event);
                        // TODO better error
                        //self.usb1
                        //    .handle_send_complete(endpoint_number)
                        //    .map_err(|_| GreatError::BadMessage)?;
                    }

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

    /// Handle any control packets that weren't handled by UsbDevice
    fn handle_control_event(
        &mut self,
        control_event: ControlEvent<{ libgreat::gcp::LIBGREAT_MAX_COMMAND_SIZE }>,
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
            "handle_control direction:{:?} packet:{:?} request_type:{:?} vendor_request:{:?} vendor_value:{:?} data:{:?}",
            direction,
            setup_packet,
            request_type,
            vendor_request,
            vendor_value,
            data,
        );

        match (&request_type, &vendor_request) {
            (RequestType::Vendor, VendorRequest::UsbCommandRequest) => {
                match (&direction, &vendor_value) {
                    // host is starting a new command sequence
                    (Direction::HostToDevice, VendorValue::Execute) => {
                        // TODO dispatch command
                        debug!("  GOT COMMAND data:{:?}", data);
                    }

                    // host is ready to receive a response
                    (Direction::DeviceToHost, VendorValue::Execute) => {
                        debug!("  GOT RESPONSE REQUEST");
                        // TODO send queued response
                        //let buf = [0_u8; 64];
                        let buf = {
                            let mut buf = [0_u8; 64];
                            for n in 0..64_usize {
                                buf[n] = n as u8;
                            }
                            buf
                        };

                        self.usb1.hal_driver.write(0, buf.into_iter());
                        self.usb1.hal_driver.write(0, buf.into_iter());
                        // end data stage with ACK ?
                        self.usb1.hal_driver.write(0, [].into_iter());
                    }

                    // host would like to abort the current command sequence
                    (Direction::DeviceToHost, VendorValue::Cancel) => {
                        debug!("  GOT ABORT");
                        // TODO
                    }

                    _ => {
                        error!(
                            "LIBGREAT stall: unknown vendor request and/or value direction{:?} vendor_request{:?} vendor_value:{:?}",
                            direction, vendor_request, vendor_value
                        );
                        self.usb1.hal_driver.stall_control_request();
                    }
                }
            }
            (RequestType::Vendor, VendorRequest::Unknown(vendor_request)) => {
                error!("LIBGREAT Unknown vendor request '{}'", vendor_request);
                return Err(GreatError::BadMessage);
            }
            _ => {
                error!("Unknown control packet '{:?}'", setup_packet);
                return Err(GreatError::BadMessage);
            }
        }

        Ok(())
    }

    // USB1_EP_OUT 2 - receive command data
    fn handle_receive_command_packet(
        &mut self,
        bytes_read: usize,
        buffer: [u8; moondancer::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        /*if bytes_read == 0 {
            // it's an ack for the last libgreat response we sent, ignore it
            return Ok(());
        }*/

        debug!("Received {} bytes on usb1 command endpoint", bytes_read,);

        if bytes_read >= 8 {
            // it's libgreat request data, dispatch it
            //self.dispatch_libgreat_bulk_request(&buffer[0..bytes_read])?;
        } else {
            warn!("  TODO this should never be called!");
        }

        Ok(())
    }

    // USB1_EP_OUT n
    //
    // This shouldn't ever be called
    fn handle_receive_packet(
        &mut self,
        endpoint: u8,
        bytes_read: usize,
        buffer: [u8; moondancer::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        error!(
            "Usb1 received {} bytes on endpoint: {}",
            bytes_read, endpoint,
        );
        error!("  TODO this should never be called!");
        Ok(())
    }

    // USB1_EP_IN n
    pub fn handle_send_complete(&mut self, endpoint: u8) -> GreatResult<()> {
        debug!("handle_send_complete(endpoint: {})", endpoint);

        Ok(())
    }
}

// - usb descriptors ----------------------------------------------------------

use moondancer::usb::{DEVICE_SERIAL_STRING, DEVICE_VERSION_NUMBER};
use smolusb::descriptor::*;

pub static DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,    // Composite
    device_subclass: 0x00, // Composite
    device_protocol: 0x00, // Composite
    max_packet_size: 64,
    vendor_id: cynthion::shared::usb::bVendorId::example,
    product_id: cynthion::shared::usb::bProductId::example,
    device_version_number: DEVICE_VERSION_NUMBER,
    manufacturer_string_index: 1,
    product_string_index: 2,
    serial_string_index: 3,
    num_configurations: 1,
    ..DeviceDescriptor::new()
};

pub static DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,    // Composite
    device_subclass: 0x00, // Composite
    device_protocol: 0x00, // Composite
    max_packet_size: 64,
    num_configurations: 1,
    ..DeviceQualifierDescriptor::new()
};

pub static CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        descriptor_type: DescriptorType::Configuration as u8,
        configuration_value: 1,
        configuration_string_index: 4,
        attributes: 0x80, // 0b1000_0000 = bus-powered
        max_power: 250,   // 250 * 2 mA = 500 mA ?
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            interface_number: 0,
            alternate_setting: 0,
            interface_class: 0xff, // Vendor-specific
            interface_subclass: cynthion::shared::usb::bInterfaceSubClass::libgreat,
            interface_protocol: cynthion::shared::usb::bInterfaceProtocol::libgreat,
            interface_string_index: 5,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                endpoint_address: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                attributes: 0x02,                                                         // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                attributes: 0x02,                                                          // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

pub static OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            descriptor_type: DescriptorType::OtherSpeedConfiguration as u8,
            configuration_value: 1,
            configuration_string_index: 7,
            attributes: 0x80, // 0b1000_0000 = bus-powered
            max_power: 250,   // 250 * 2 mA = 500 mA ?
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                interface_number: 0,
                alternate_setting: 0,
                interface_class: 0xff, // Vendor-specific
                interface_subclass: cynthion::shared::usb::bInterfaceSubClass::moondancer,
                interface_protocol: cynthion::shared::usb::bInterfaceProtocol::moondancer,
                interface_string_index: 5,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    endpoint_address: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                    attributes: 0x02, // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                    attributes: 0x02, // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

pub static USB_STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

pub static USB_STRING_DESCRIPTOR_1: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bManufacturerString::cynthion); // manufacturer
pub static USB_STRING_DESCRIPTOR_2: StringDescriptor =
    StringDescriptor::new("test_usb.py unittest runner"); // product
pub static USB_STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new(DEVICE_SERIAL_STRING); // serial
pub static USB_STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("config0"); // configuration #0
pub static USB_STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("interface0"); // interface #0
pub static USB_STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("interface1"); // interface #1
pub static USB_STRING_DESCRIPTOR_7: StringDescriptor = StringDescriptor::new("config1"); // configuration #1

pub static USB_STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &USB_STRING_DESCRIPTOR_1,
    &USB_STRING_DESCRIPTOR_2,
    &USB_STRING_DESCRIPTOR_3,
    &USB_STRING_DESCRIPTOR_4,
    &USB_STRING_DESCRIPTOR_5,
    &USB_STRING_DESCRIPTOR_6,
    &USB_STRING_DESCRIPTOR_7,
];
