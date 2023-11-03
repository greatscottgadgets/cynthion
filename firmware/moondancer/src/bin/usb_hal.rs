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

const DEVICE_SPEED: Speed = Speed::Full;

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

    let usb0 = unsafe { hal::Usb0::summon() };

    // - usb0 interrupts - "target_phy" --

    // USB0 BusReset
    if usb0.is_pending(pac::Interrupt::USB0) {
        ladybug::trace(Channel::A, 0, || {
            usb0.bus_reset();
            dispatch_event(InterruptEvent::Usb(Target, UsbEvent::BusReset));
        });
        usb0.clear_pending(pac::Interrupt::USB0);

    // USB0_EP_CONTROL ReceiveControl
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        ladybug::trace(Channel::A, 1, || {
            /*let endpoint = usb0.ep_control.epno.read().bits() as u8;
            usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
            dispatch_event(InterruptEvent::Usb(
            Target,
            UsbEvent::ReceiveControl(endpoint),
            ));*/

            let endpoint = usb0.ep_control.epno.read().bits() as u8;
            let mut buffer = [0_u8; 8];
            let _bytes_read = usb0.read_control(&mut buffer);
            let setup_packet = SetupPacket::from(buffer);
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceiveSetupPacket(endpoint, setup_packet),
            ));
        });
        usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);

        // USB0_EP_OUT ReceivePacket
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        ladybug::trace(Channel::A, 2, || {
            let endpoint = usb0.ep_out.data_ep.read().bits() as u8;
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceivePacket(endpoint),
            ));
        });
        usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);

    // USB0_EP_IN SendComplete
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        ladybug::trace(Channel::A, 3, || {
            let endpoint = usb0.ep_in.epno.read().bits() as u8;
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::SendComplete(endpoint),
            ));
        });
        usb0.clear_pending(pac::Interrupt::USB0_EP_IN);

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

    // usb0: Target
    let usb0 = hal::Usb0::new(
        peripherals.USB0,
        peripherals.USB0_EP_CONTROL,
        peripherals.USB0_EP_IN,
        peripherals.USB0_EP_OUT,
    );

    // control
    let mut control = Control::<_, { moondancer::EP_MAX_PACKET_SIZE }>::new(
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
    usb0.set_speed(DEVICE_SPEED);

    // connect device
    usb0.connect();
    info!("Connected usb0 device");

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable usb0 interrupts and events
        pac::csr::interrupt::enable(pac::Interrupt::USB0);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_IN);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_OUT);
        usb0.enable_interrupts();
    }

    // prime the usb OUT endpoint(s) we'll be using
    usb0.ep_out_prime_receive(1);
    usb0.ep_out_prime_receive(2);

    use moondancer::{event::InterruptEvent::*, UsbInterface::Target};
    use smolusb::event::UsbEvent::*;

    info!("Peripherals initialized, entering main loop.");

    loop {
        let event = EVENT_QUEUE.dequeue();
        if matches!(event, None) {
            continue;
        }
        let event = event.unwrap();

        // Usb0 received a control event
        match event {
            Usb(Target, event @ BusReset)                 |
            Usb(Target, event @ ReceiveControl(0))        |
            Usb(Target, event @ ReceiveSetupPacket(0, _)) |
            Usb(Target, event @ ReceivePacket(0))         |
            Usb(Target, event @ SendComplete(0)) => {
                ladybug::trace(Channel::B, 0, || {
                    control.handle_event(&usb0, event);
                });
            }
            Usb(Target, ReceivePacket(endpoint)) => {
                log::info!("USB0 Event: {:?}", event);
            }
            Usb(Target, SendComplete(_endpoint)) => {
                log::info!("USB0 Event: {:?}", event);
            }
            _ => {
                error!("Unhandled event: {:?}", event);
            }
        }

        /*if let Usb(Target, usb_event) = event {
            ladybug::trace(Channel::B, 0, || {
                control.handle_event(&usb0, usb_event);
            });
        }*/

        /*match event {
            // USB0 - bus reset
            Usb(Target, BusReset) => {
                // handled in MachineExternal
            }

            // USB0_EP_CONTROL - received a control event
            //Usb(Target, ReceiveControl(endpoint)) => {
            Usb(Target, ReceiveSetupPacket(endpoint, setup_packet)) => {
                //let mut buffer = [0_u8; 8];
                //let _bytes_read = usb0.read_control(&mut buffer);

                // parse setup packet
                //let setup_packet = SetupPacket::from(buffer);
                let request_type = setup_packet.request_type();
                let request = setup_packet.request();

                //info!("  SETUP {:?}", request);

                // handle setup request
                match (&request_type, &request) {
                    (RequestType::Standard, Request::SetAddress) => {
                        let address: u8 = (setup_packet.value & 0x7f) as u8;

                        // set tx_ack_active flag
                        // TODO a slighty safer approach would be nice
                        unsafe {
                            usb0.set_tx_ack_active();
                        }

                        // end status stage first before changing device address
                        //usb0.ack_status_stage(&setup_packet);
                        // send ZLP to host to end status stage
                        usb0.ack(0, Direction::HostToDevice);

                        // wait for SendComplete
                        // TODO a slightly safer approach would be nice
                        loop {
                            let active = unsafe { usb0.is_tx_ack_active() };
                            if active == false {
                                break;
                            }
                        }

                        // activate new address
                        usb0.set_address(address);
                    }
                    (RequestType::Standard, Request::GetDescriptor) => {
                        // extract the descriptor type and number from our SETUP request
                        let [descriptor_number, descriptor_type] = setup_packet.value.to_le_bytes();
                        let descriptor_type = match DescriptorType::try_from(descriptor_type) {
                            Ok(descriptor_type) => descriptor_type,
                            Err(e) => {
                                warn!(
                                    "USB0_EP_CONTROL stall: invalid descriptor type: {} {}",
                                    descriptor_type, descriptor_number
                                );
                                usb0.stall_control_request();
                                continue;
                            }
                        };

                        //info!("    GetDescriptor {:?} {}", descriptor_type, descriptor_number);

                        // if the host is requesting less than the maximum amount of data,
                        // only respond with the amount requested
                        let requested_length = setup_packet.length as usize;

                        // respond with the requested descriptor
                        match (&descriptor_type, descriptor_number) {
                            (DescriptorType::Device, 0) => {
                                usb0.write_ref(
                                    endpoint,
                                    USB_DEVICE_DESCRIPTOR.as_iter().take(requested_length),
                                );
                            }
                            (DescriptorType::Configuration, 0) => {
                                usb0.write_ref(
                                    endpoint,
                                    usb_configuration_descriptor_0.iter().take(requested_length),
                                );
                            }
                            (DescriptorType::DeviceQualifier, 0) => {
                                if DEVICE_SPEED == Speed::High {
                                    usb0.write_ref(
                                        endpoint,
                                        USB_DEVICE_QUALIFIER_DESCRIPTOR
                                            .as_iter()
                                            .take(requested_length),
                                    );
                                } else {
                                    usb0.ack(0, Direction::HostToDevice);
                                }
                            }
                            (DescriptorType::OtherSpeedConfiguration, 0) => {
                                info!("OtherSpeedConfiguration");
                                // optional
                                /*warn!("USB0_EP_CONTROL stall: no other speed configuration descriptor");
                                //usb0.stall_endpoint_out(endpoint);
                                //usb0.ack_status_stage(&setup_packet);
                                usb0.ack(0, Direction::HostToDevice);*/

                                usb0.write_ref(
                                    endpoint,
                                    usb_other_speed_configuration_descriptor_0
                                        .iter()
                                        .take(requested_length),
                                );
                            }
                            (DescriptorType::String, 0) => {
                                usb0.write_ref(
                                    endpoint,
                                    USB_STRING_DESCRIPTOR_0.iter().take(requested_length),
                                );
                            }
                            (DescriptorType::String, index) => {
                                let offset_index: usize = (index - 1).into();
                                if offset_index > USB_STRING_DESCRIPTORS.len() {
                                    warn!(
                                        "USB0_EP_CONTROL stall: unknown string descriptor {}",
                                        index
                                    );
                                    usb0.stall_control_request();
                                    continue;
                                }
                                usb0.write(
                                    endpoint,
                                    USB_STRING_DESCRIPTORS[offset_index]
                                        .iter()
                                        .take(requested_length),
                                );
                            }
                            _ => {
                                warn!(
                                    "USB0_EP_CONTROL stall: unhandled descriptor request {:?}, {}",
                                    descriptor_type, descriptor_number
                                );
                                usb0.stall_control_request();
                                continue;
                            }
                        }

                        // finally, ack status stage - all this does for DeviceToHost is to prime
                        // ep_out 0 to receive the zlp from the host
                        //usb0.ack_status_stage(&setup_packet);
                        usb0.ack(0, Direction::DeviceToHost); // host will send zlp
                    }
                    (RequestType::Standard, Request::SetConfiguration) => {
                        let configuration: u8 = setup_packet.value as u8;
                        info!("Request::SetConfiguration {}", configuration);

                        if configuration > 1 {
                            warn!(
                                "USB0_EP_CONTROL stall: unknown configuration {}",
                                configuration
                            );
                            current_configuration = 0;
                            usb0.stall_control_request();
                            continue;
                        } else {
                            current_configuration = configuration;
                        }

                        //usb0.ack_status_stage(&setup_packet); // TODO immediately, really?
                        usb0.ack(0, Direction::HostToDevice); // device sends zlp
                    }
                    (RequestType::Standard, Request::GetConfiguration) => {
                        info!("Request::GetConfiguration");
                        usb0.write_ref(0, [current_configuration].iter());
                        usb0.ack_status_stage(&setup_packet); // TODO direction?
                    }
                    (RequestType::Standard, Request::ClearFeature) => {
                        info!("Request::ClearFeature");
                        let recipient = setup_packet.recipient();
                        let feature_bits = setup_packet.value;
                        let feature = match Feature::try_from(feature_bits) {
                            Ok(feature) => feature,
                            Err(e) => {
                                warn!(
                                    "USB0_EP_CONTROL stall: invalid clear feature type: {}",
                                    feature_bits
                                );
                                usb0.stall_control_request();
                                continue;
                            }
                        };

                        use smolusb::setup::{Feature, Recipient};
                        match (&recipient, &feature) {
                            (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                                // self.feature_remote_wakeup = false;
                            }
                            (Recipient::Endpoint, Feature::EndpointHalt) => {
                                let endpoint_address = setup_packet.index as u8;
                                usb0
                                    .clear_feature_endpoint_halt(endpoint_address);
                                usb0.ack_status_stage(&setup_packet);
                            }
                            _ => {
                                warn!(
                                    "USB0_EP_CONTROL stall: unhandled clear feature {:?}, {:?}",
                                    recipient, feature
                                );
                                usb0.stall_control_request();
                                return Ok(());
                            }
                        };
                    }
                    (RequestType::Standard, Request::SetFeature) => {
                        info!("Request::SetFeature");
                        let recipient = setup_packet.recipient();
                        let feature_bits = setup_packet.value;
                        let feature = match Feature::try_from(feature_bits) {
                            Ok(feature) => feature,
                            Err(e) => {
                                warn!(
                                    "USB0_EP_CONTROL stall: invalid clear feature type: {}",
                                    feature_bits
                                );
                                usb0.stall_control_request();
                                continue;
                            }
                        };
                        use smolusb::setup::{Feature, Recipient};
                        match (&recipient, &feature) {
                            (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                                // self.feature_remote_wakeup = true;
                            }
                            _ => {
                                warn!(
                                    "USB0_EP_CONTROL stall: unhandled set feature {:?}, {:?}",
                                    recipient, feature
                                );
                                usb0.stall_control_request();
                                return Ok(());
                            }
                        };

                        // TODO ack?
                    }
                    (RequestType::Standard, Request::GetStatus) => {
                        info!("Request::GetStatus");
                        let _recipient = setup_packet.recipient();
                        let status: u16 = 0b00; // TODO bit 1:remote-wakeup bit 0:self-powered
                        usb0.write_ref(0, status.to_le_bytes().iter());
                        usb0.ack_status_stage(&setup_packet);
                    }
                    _ => {
                        log::warn!(
                            "USB_EP_CONTROL unhandled request {:?} {:?}",
                            request_type,
                            request
                        );
                    }
                }
            }

            // USB0_EP_OUT n - received packet on endpoint
            Usb(Target, ReceivePacket(endpoint)) => {
                let mut rx_buffer: [u8; moondancer::EP_MAX_PACKET_SIZE] =
                    [0; moondancer::EP_MAX_PACKET_SIZE];
                let bytes_read = usb0.read(endpoint, &mut rx_buffer);
                if bytes_read == 0 {
                    // it's an ack
                } else {
                    info!(
                        "USB0_EP_OUT received packet on endpoint:{} bytes_read:{}",
                        endpoint, bytes_read
                    );
                }
                usb0.ep_out_prime_receive(endpoint);
            }

            // USB0_EP_IN - transfer complete
            Usb(Target, SendComplete(_endpoint)) => {}

            // Error Message
            ErrorMessage(message) => {
                error!("MachineExternal Error - {}", message);
            }

            // Unhandled event
            _ => {
                error!("Unhandled event: {:?}", event);
            }
        } // end match
        */

    } // end loop
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
