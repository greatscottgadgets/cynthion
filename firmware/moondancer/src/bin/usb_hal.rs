#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![no_std]
#![no_main]

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info, warn};

use libgreat::{GreatError, GreatResult};

use smolusb::control_new::{Control, Descriptors};
use smolusb::descriptor::*;
use smolusb::device::{Speed, UsbDevice};
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, Request, RequestType, SetupPacket};
use smolusb::traits::AsByteSliceIterator;
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
    WriteRefEndpoint,
};

use moondancer::event::InterruptEvent;
use moondancer::{hal, pac};

use ladybug::Channel;

// - configuration ------------------------------------------------------------

const DEVICE_SPEED: Speed = Speed::High;

// - global static state ------------------------------------------------------

static EVENT_QUEUE: Queue<InterruptEvent, 32> = Queue::new();

#[inline(always)]
fn dispatch_event(event: InterruptEvent) {
    match EVENT_QUEUE.enqueue(event) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - event queue overflow");
            panic!("MachineExternal - event queue overflow");
        }
    }
}

// - MachineExternal interrupt handler ----------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    use moondancer::UsbInterface::Target;

    let usb1 = unsafe { hal::Usb1::summon() };

    // - usb1 interrupts - "aux_phy" --

    // USB1 BusReset
    if usb1.is_pending(pac::Interrupt::USB1) {
        ladybug::trace(Channel::A, 0, || {
            usb1.bus_reset();
            dispatch_event(InterruptEvent::Usb(Target, UsbEvent::BusReset));

            usb1.clear_pending(pac::Interrupt::USB1);
        });

    // USB1_EP_CONTROL ReceiveControl
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        ladybug::trace(Channel::A, 1, || {
            let endpoint = usb1.ep_control.epno.read().bits() as u8;

            #[cfg(not(feature="chonky_events"))]
            {
                dispatch_event(InterruptEvent::Usb(
                    Target,
                    UsbEvent::ReceiveControl(endpoint),
                ));
            }

            #[cfg(feature="chonky_events")]
            {
                let endpoint = usb1.ep_control.epno.read().bits() as u8;
                let mut buffer = [0_u8; 8];
                let _bytes_read = usb1.read_control(&mut buffer);
                let setup_packet = SetupPacket::from(buffer);
                dispatch_event(InterruptEvent::Usb(
                    Target,
                    UsbEvent::ReceiveSetupPacket(endpoint, setup_packet),
                ));
            }

            usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        });

    // USB1_EP_OUT ReceivePacket
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        ladybug::trace(Channel::A, 2, || {
            let endpoint = usb1.ep_out.data_ep.read().bits() as u8;

            #[cfg(not(feature="chonky_events"))]
            {
                dispatch_event(InterruptEvent::Usb(
                    Target,
                    UsbEvent::ReceivePacket(endpoint),
                ));
            }

            #[cfg(feature="chonky_events")]
            {
                // #1 empty fifo into a receive buffer
                let mut packet_buffer: [u8; moondancer::EP_MAX_PACKET_SIZE] = [0; moondancer::EP_MAX_PACKET_SIZE];
                let bytes_read = usb1.read(endpoint, &mut packet_buffer);
                // pulse zlp reads
                if bytes_read == 0 {
                    ladybug::trace(Channel::B, 7, || {
                    });
                }

                // #2 dispatch receive buffer to the main loop
                dispatch_event(InterruptEvent::Usb(
                    Target,
                    UsbEvent::ReceiveBuffer(endpoint, bytes_read, packet_buffer),
                ));

                // #3 tell eptri we're ready to receive another packet
                //usb1.ep_out_prime_receive(endpoint);
            }

            // #4 tell the cpu we're ready for another rx interrupt
            usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);
        });

    // USB1_EP_IN SendComplete
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        ladybug::trace(Channel::A, 3, || {
            let endpoint = usb1.ep_in.epno.read().bits() as u8;
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::SendComplete(endpoint),
            ));

            usb1.clear_pending(pac::Interrupt::USB1_EP_IN);
        });

    // - Unknown Interrupt --
    } else {
        let pending = pac::csr::interrupt::reg_pending();
        dispatch_event(InterruptEvent::UnknownInterrupt(pending));
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
    match main_loop() {
        Ok(()) => {
            error!("Firmware exited unexpectedly in main loop");
            panic!("Firmware exited unexpectedly in main loop")
        }
        Err(e) => {
            error!("Fatal error in firmware main loop: {}", e);
            panic!("Fatal error in firmware main loop: {}", e)
        }
    }
}

// - main loop ----------------------------------------------------------------

fn main_loop() -> GreatResult<()> {
    let peripherals = pac::Peripherals::take().unwrap();

    // initialize logging
    moondancer::log::init(hal::Serial::new(peripherals.UART));
    info!("Logging initialized");

    // initialize ladybug
    moondancer::debug::init(peripherals.GPIOA, peripherals.GPIOB);

    // usb1: Target
    let usb1 = hal::Usb1::new(
        peripherals.USB1,
        peripherals.USB1_EP_CONTROL,
        peripherals.USB1_EP_IN,
        peripherals.USB1_EP_OUT,
    );

    // control
    let mut control = Control::<_, { moondancer::EP_MAX_PACKET_SIZE * 4 }>::new(
        0,
        Descriptors {
            device_speed: DEVICE_SPEED,
            device_descriptor: USB_DEVICE_DESCRIPTOR,
            configuration_descriptor: USB_CONFIGURATION_DESCRIPTOR_0,
            other_speed_configuration_descriptor: Some(USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(USB_DEVICE_QUALIFIER_DESCRIPTOR),
            string_descriptor_zero: USB_STRING_DESCRIPTOR_0,
            string_descriptors: USB_STRING_DESCRIPTORS,
        }//.set_total_lengths() // TODO figure out a better solution
    );

    // set controller speed
    usb1.set_speed(DEVICE_SPEED);

    // connect device
    usb1.connect();
    info!("Connected usb1 device");

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable usb1 interrupts and events
        pac::csr::interrupt::enable(pac::Interrupt::USB1);
        pac::csr::interrupt::enable(pac::Interrupt::USB1_EP_CONTROL);
        pac::csr::interrupt::enable(pac::Interrupt::USB1_EP_IN);
        pac::csr::interrupt::enable(pac::Interrupt::USB1_EP_OUT);
        usb1.enable_interrupts();
    }

    // prime the usb OUT endpoint(s) we'll be using
    //usb1.ep_out_prime_receive(0);
    //usb1.ep_out_prime_receive(1);

    use moondancer::{event::InterruptEvent::*, UsbInterface::Target};
    use smolusb::event::UsbEvent::*;

    info!("Peripherals initialized, entering main loop.");

    loop {

        // 100uS from interrupt to dequeued
        if let Some(event) = EVENT_QUEUE.dequeue() {
            // Usb1 received a control event
            match event {
                #[cfg(feature="chonky_events")]
                Usb(Target, event @ BusReset)                 |
                Usb(Target, event @ ReceiveControl(0))        |
                Usb(Target, event @ ReceiveSetupPacket(0, _)) |
                Usb(Target, event @ ReceivePacket(0))         |
                Usb(Target, event @ ReceiveBuffer(0, _, _))   |
                Usb(Target, event @ SendComplete(0)) => {
                    let result = ladybug::trace(Channel::B, 0, || {
                        control.handle_event(&usb1, event)
                    });
                    match result {
                        // vendor requests are not handled by control
                        Some((setup_packet, rx_buffer)) => {
                            ladybug::trace(Channel::B, 6, || {
                                handle_vendor_request(&usb1, setup_packet, rx_buffer);
                            });
                        }
                        // control event was handled
                        None => (),
                    }
                }
                #[cfg(not(feature="chonky_events"))]
                Usb(Target, event @ BusReset)                 |
                Usb(Target, event @ ReceiveControl(0))        |
                Usb(Target, event @ ReceivePacket(0))         |
                Usb(Target, event @ SendComplete(0)) => {
                    let result = ladybug::trace(Channel::B, 0, || {
                        control.handle_event(&usb1, event)
                    });
                    match result {
                        // vendor requests are not handled by control
                        Some((setup_packet, rx_buffer)) => {
                            ladybug::trace(Channel::B, 6, || {
                                handle_vendor_request(&usb1, setup_packet, rx_buffer);
                            });
                        }
                        // control event was handled
                        None => (),
                    }
                }
                Usb(Target, ReceivePacket(endpoint)) => {
                    log::info!("USB1 Event: {:?}", event);
                }
                Usb(Target, SendComplete(_endpoint)) => {
                    log::info!("USB1 Event: {:?}", event);
                }
                _ => {
                    error!("Unhandled event: {:?}", event);
                }
            }
        }
    } // end loop
}

// - vendor request handler ---------------------------------------------------

fn handle_vendor_request<'a, D>(
    usb: &D,
    setup_packet: SetupPacket,
    rx_buffer: &[u8],
) where
    D: ReadControl + ReadEndpoint + WriteEndpoint + WriteRefEndpoint + UsbDriverOperations,
{
    let direction = setup_packet.direction();
    let request_type = setup_packet.request_type();
    let vendor_request = setup_packet.request;
    let vendor_value = setup_packet.value;

    /*if rx_buffer.len() != 518 {
        error!("handle_vendor_request() unexpected transfer length of {} bytes", rx_buffer.len());
    }*/

    info!(
        "handle_vendor_request: {:?} {:?} vendor_request:{} vendor_value:{} rx_buffer:{}",
        direction, request_type, vendor_request, vendor_value, rx_buffer.len()
    );

    if rx_buffer.len() > 0 {
        info!("{:?} ... {:?}", &rx_buffer[..8], &rx_buffer[rx_buffer.len()-8..]);
        //let bytes_written = usb.write_packets(0, rx_buffer.iter().cloned(), 64);
        //let bytes_written = usb.write_ref(0, rx_buffer.iter());
        //info!("Wrote {} bytes", bytes_written);
    }

}


// - usb descriptors ----------------------------------------------------------

static USB_DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,
    device_subclass: 0x00,
    device_protocol: 0x00,
    max_packet_size: 64,
    vendor_id: cynthion::shared::usb::bVendorId::example,
    product_id: cynthion::shared::usb::bProductId::example,
    device_version_number: moondancer::usb::DEVICE_VERSION_NUMBER,
    manufacturer_string_index: 1,
    product_string_index: 2,
    serial_string_index: 3,
    num_configurations: 1,
    ..DeviceDescriptor::new()
};

static USB_DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,
    device_subclass: 0x00,
    device_protocol: 0x00,
    max_packet_size: 64,
    num_configurations: 1,
    ..DeviceQualifierDescriptor::new()
};

static USB_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        configuration_value: 1,
        configuration_string_index: 4,
        attributes: 0x80, // 0b1000_0000 = bus-powered
        max_power: 50,    // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            interface_number: 0,
            alternate_setting: 0,
            interface_class: 0x00,
            interface_subclass: 0x00,
            interface_protocol: 0x00,
            interface_string_index: 5,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                endpoint_address: 0x01, // OUT
                attributes: 0x02,       // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: 0x02, // OUT - host commands
                attributes: 0x02,       // Bulk
                max_packet_size: 8,
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: 0x81, // IN
                attributes: 0x02,       // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

static USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            descriptor_type: DescriptorType::OtherSpeedConfiguration as u8,
            configuration_value: 1,
            configuration_string_index: 7,
            attributes: 0x80, // 0b1000_0000 = bus-powered
            max_power: 50,    // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                interface_number: 0,
                alternate_setting: 0,
                interface_class: 0x00,
                interface_subclass: 0x00,
                interface_protocol: 0x00,
                interface_string_index: 5,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    endpoint_address: 0x01, // OUT
                    attributes: 0x02,       // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: 0x02, // OUT - host commands
                    attributes: 0x02,       // Bulk
                    max_packet_size: 8,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: 0x81, // IN
                    attributes: 0x02,       // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

static USB_STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);
static USB_STRING_DESCRIPTOR_1: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bManufacturerString::example);
static USB_STRING_DESCRIPTOR_2: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bProductString::example);
static USB_STRING_DESCRIPTOR_3: StringDescriptor =
    StringDescriptor::new(moondancer::usb::DEVICE_SERIAL_STRING);
pub static USB_STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("config0"); // configuration #0
pub static USB_STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("interface0"); // interface #0
pub static USB_STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("interface1"); // interface #1
pub static USB_STRING_DESCRIPTOR_7: StringDescriptor = StringDescriptor::new("config1"); // configuration #1


static USB_STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &USB_STRING_DESCRIPTOR_1,
    &USB_STRING_DESCRIPTOR_2,
    &USB_STRING_DESCRIPTOR_3,
    &USB_STRING_DESCRIPTOR_4,
    &USB_STRING_DESCRIPTOR_5,
    &USB_STRING_DESCRIPTOR_6,
    &USB_STRING_DESCRIPTOR_7,
];
