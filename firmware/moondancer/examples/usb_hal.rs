#![no_std]
#![no_main]

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info};

use libgreat::GreatResult;

use crate::hal::smolusb;
use smolusb::control::Control;
use smolusb::descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptorHeader, DescriptorType, DeviceDescriptor,
    DeviceQualifierDescriptor, EndpointDescriptor, InterfaceDescriptor, InterfaceDescriptorHeader,
    LanguageId, StringDescriptor, StringDescriptorZero,
};
use smolusb::device::{Descriptors, Speed};
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, SetupPacket};
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
};

use moondancer::event::InterruptEvent;
use moondancer::{hal, pac};

// - configuration ------------------------------------------------------------

const DEVICE_SPEED: Speed = Speed::High;

const VENDOR_REQUEST: u8 = 0x65;
const VENDOR_CONTROL_OUT: u16 = 0x0001;
const VENDOR_CONTROL_IN: u16 = 0x0002;
const VENDOR_BULK_OUT: u16 = 0x0003;
const VENDOR_BULK_IN: u16 = 0x0004;

const ENDPOINT_BULK_OUT: u8 = 0x01;
const ENDPOINT_BULK_IN: u8 = 0x81;

const MAX_TRANSFER_SIZE: usize = smolusb::EP_MAX_PACKET_SIZE * 4;

// - global static state ------------------------------------------------------

static EVENT_QUEUE: Queue<InterruptEvent, 64> = Queue::new();

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
extern "C" fn MachineExternal() {
    use moondancer::UsbInterface::Target;

    let usb0 = unsafe { hal::Usb0::summon() };

    let pending = match pac::csr::interrupt::pending() {
        Ok(interrupt) => interrupt,
        Err(pending) => {
            dispatch_event(InterruptEvent::UnknownInterrupt(pending));
            return;
        }
    };

    match pending {
        // - usb0 interrupts - "target_phy" --

        // USB0 BusReset
        pac::Interrupt::USB0 => {
            usb0.controller
                .ev_pending()
                .modify(|r, w| w.pending().bit(r.pending().bit()));

            // handle bus reset in interrupt handler for lowest latency
            usb0.bus_reset();
            dispatch_event(InterruptEvent::Usb(Target, UsbEvent::BusReset));
        }
        // USB0_EP_CONTROL ReceiveControl
        pac::Interrupt::USB0_EP_CONTROL => {
            usb0.ep_control
                .ev_pending()
                .modify(|r, w| w.pending().bit(r.pending().bit()));

            let endpoint = usb0.ep_control.epno().read().bits() as u8;
            let mut buffer = [0_u8; 8];
            let _bytes_read = usb0.read_control(&mut buffer);
            let setup_packet = SetupPacket::from(buffer);
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceiveSetupPacket(endpoint, setup_packet),
            ));
        }
        // USB0_EP_IN SendComplete
        pac::Interrupt::USB0_EP_IN => {
            usb0.ep_in
                .ev_pending()
                .modify(|r, w| w.pending().bit(r.pending().bit()));

            // TODO something a little safer would be nice
            let endpoint = usb0.ep_in.epno().read().bits() as u8;
            unsafe {
                usb0.clear_tx_ack_active(endpoint);
            }

            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::SendComplete(endpoint),
            ));
        }
        // USB0_EP_OUT ReceivePacket
        pac::Interrupt::USB0_EP_OUT => {
            usb0.ep_out
                .ev_pending()
                .modify(|r, w| w.pending().bit(r.pending().bit()));

            let endpoint = usb0.ep_out.data_ep().read().bits() as u8;
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceivePacket(endpoint),
            ));
        }

        // - Unhandled Interrupt --
        _ => dispatch_event(InterruptEvent::UnhandledInterrupt(pending)),
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
    moondancer::log::init();
    info!("Logging initialized");

    // usb0: Target
    let mut usb0 = hal::Usb0::new(
        peripherals.USB0,
        peripherals.USB0_EP_CONTROL,
        peripherals.USB0_EP_IN,
        peripherals.USB0_EP_OUT,
    );

    // control
    let mut control = Control::<_, MAX_TRANSFER_SIZE>::new(
        0,
        Descriptors {
            device_speed: DEVICE_SPEED,
            device_descriptor: USB_DEVICE_DESCRIPTOR,
            configuration_descriptor: USB_CONFIGURATION_DESCRIPTOR_0,
            other_speed_configuration_descriptor: Some(USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(USB_DEVICE_QUALIFIER_DESCRIPTOR),
            string_descriptor_zero: USB_STRING_DESCRIPTOR_0,
            string_descriptors: USB_STRING_DESCRIPTORS,
        }, //.set_total_lengths() // TODO figure out a better solution
    );

    // connect device
    usb0.connect(DEVICE_SPEED);
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
        usb0.enable_events();
    }

    // prime the usb Bulk OUT endpoint(s) we'll be using
    //usb0.ep_out_prime_receive(ENDPOINT_BULK_OUT);

    info!("Peripherals initialized, entering main loop.");

    loop {
        use moondancer::{event::InterruptEvent::*, UsbInterface::Target};
        use smolusb::event::UsbEvent::*;

        // 100uS from interrupt to dequeued
        if let Some(event) = EVENT_QUEUE.dequeue() {
            // Usb0 received a control event
            match event {
                Usb(
                    Target,
                    event @ (BusReset
                    | ReceiveSetupPacket(0, _)
                    | ReceivePacket(0)
                    | SendComplete(0)),
                ) => {
                    if let Some(setup_packet) = control.dispatch_event(&usb0, event) {
                        // vendor requests are not handled by control
                        handle_vendor_request(&usb0, setup_packet, control.data());
                    }
                }
                Usb(Target, ReceivePacket(endpoint @ ENDPOINT_BULK_OUT)) => {
                    let mut rx_buffer: [u8; smolusb::EP_MAX_PACKET_SIZE] =
                        [0; smolusb::EP_MAX_PACKET_SIZE];
                    let bytes_read = usb0.read(endpoint, &mut rx_buffer);
                    debug!("VENDOR_BULK_OUT received {} bytes", bytes_read);
                }
                Usb(Target, SendComplete(_endpoint)) => {
                    log::debug!("USB0 Event: {:?}", event);
                }
                _ => {
                    error!("Unhandled event: {:?}", event);
                }
            }
        }
    } // end loop
}

// - vendor request handler ---------------------------------------------------

fn handle_vendor_request<D>(usb: &D, setup_packet: SetupPacket, rx_buffer: &[u8])
where
    D: ReadControl + ReadEndpoint + WriteEndpoint + UsbDriverOperations + UnsafeUsbDriverOperations,
{
    let direction = setup_packet.direction();
    let request_type = setup_packet.request_type();
    let vendor_request = setup_packet.request;
    let vendor_value = setup_packet.value;
    let payload_length = setup_packet.index as usize;

    debug!(
        "handle_vendor_request: {:?} {:?} vendor_request:{} vendor_value:{} payload_length:{} rx_buffer:{}",
        direction,
        request_type,
        vendor_request,
        vendor_value,
        payload_length,
        rx_buffer.len()
    );

    match (vendor_request, vendor_value) {
        (VENDOR_REQUEST, VENDOR_CONTROL_OUT) => {
            // TODO would it be better if the caller sent the zlp at this point rather than control?
            // there's currently a subtle bug where zlp is automatic if control transfer had data
            // but caller has to send zlp themselves if there was no data.
            // really, either control has to always zlp or the caller has to always zlp
            if rx_buffer.len() == payload_length {
                debug!("VENDOR_CONTROL_OUT received {} bytes", rx_buffer.len());
            } else {
                error!(
                    "VENDOR_CONTROL_OUT expected {} bytes but only received {} bytes.",
                    payload_length,
                    rx_buffer.len()
                );
            }
        }
        (VENDOR_REQUEST, VENDOR_CONTROL_IN) => {
            let test_data: [u8; MAX_TRANSFER_SIZE] = core::array::from_fn(|x| x as u8);
            let test_data = test_data.iter().take(payload_length);

            // send requested data
            let bytes_written = usb.write(0, test_data.copied());

            // prime endpoint to receive zlp ack from host - this makes no sense or does control have a zlp???
            usb.ack(0, Direction::DeviceToHost);

            if bytes_written == payload_length {
                debug!("VENDOR_CONTROL_IN wrote {} bytes", bytes_written);
            } else {
                error!(
                    "VENDOR_CONTROL_IN payload length is {} bytes but only wrote {} bytes",
                    payload_length, bytes_written
                );
            }
        }
        (VENDOR_REQUEST, VENDOR_BULK_OUT) => {
            // prime bulk endpoint to receive data
            usb.ep_out_prime_receive(ENDPOINT_BULK_OUT);

            // send zlp response because there was no data TODO see above
            usb.ack(0, Direction::HostToDevice);

            debug!(
                "VENDOR_BULK_OUT expecting {} bytes ({})",
                payload_length,
                rx_buffer.len()
            );
        }
        (VENDOR_REQUEST, VENDOR_BULK_IN) => {
            let endpoint_number = ENDPOINT_BULK_IN & 0x7f;
            #[allow(clippy::cast_possible_truncation)]
            let test_data: [u8; MAX_TRANSFER_SIZE] = core::array::from_fn(|x| (x & 0xff) as u8);
            let test_data = test_data.iter().take(payload_length);

            // send zlp response because there was no data TODO see above
            unsafe {
                usb.set_tx_ack_active(0);
            }
            usb.ack(0, Direction::HostToDevice);

            // wait for zlp to be sent
            let mut timeout = 0;
            while unsafe { usb.is_tx_ack_active(0) } {
                timeout += 1;
                if timeout > 5_000_000 {
                    error!("VENDOR_BULK_IN timed out sending control ack");
                    return;
                }
            }

            // send requested data
            let bytes_written = usb.write(endpoint_number, test_data.copied());

            // prime endpoint to receive zlp ack from host - this makes no sense or does bulk have a zlp???
            //usb.ack(endpoint_number, Direction::DeviceToHost);

            if bytes_written == payload_length {
                debug!("VENDOR_BULK_IN wrote {} bytes", bytes_written);
            } else {
                error!(
                    "VENDOR_BULK_IN payload length is {} bytes but only wrote {} bytes",
                    payload_length, bytes_written
                );
            }
        }
        _ => {
            error!(
                "Unknown vendor_request:{} vendor_value:{}",
                vendor_request, vendor_value,
            );
        }
    }
}

// - usb descriptors ----------------------------------------------------------

const DESCRIPTOR_MAX_PACKET_SIZE: u16 = if matches!(DEVICE_SPEED, Speed::High) {
    512
} else {
    64
};
const OTHER_DESCRIPTOR_MAX_PACKET_SIZE: u16 = if matches!(DEVICE_SPEED, Speed::High) {
    64
} else {
    512
};

static USB_DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize: 64,
    idVendor: cynthion::shared::usb::bVendorId::example,
    idProduct: cynthion::shared::usb::bProductId::example,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
    ..DeviceDescriptor::new()
};

static USB_DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize0: 64,
    bNumConfigurations: 1,
    ..DeviceQualifierDescriptor::new()
};

static USB_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        bConfigurationValue: 1,
        iConfiguration: 4,
        bmAttributes: 0x80, // 0b1000_0000 = bus-powered
        bMaxPower: 50,      // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            iInterfaceNumber: 0,
            bAlternateSetting: 0,
            bInterfaceClass: 0x00,
            bInterfaceSubClass: 0x00,
            bInterfaceProtocol: 0x00,
            iInterface: 5,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                bEndpointAddress: ENDPOINT_BULK_OUT,
                bmAttributes: 0x02, // Bulk
                wMaxPacketSize: DESCRIPTOR_MAX_PACKET_SIZE,
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                bEndpointAddress: ENDPOINT_BULK_IN,
                bmAttributes: 0x02, // Bulk
                wMaxPacketSize: DESCRIPTOR_MAX_PACKET_SIZE,
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

static USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            bDescriptorType: DescriptorType::OtherSpeedConfiguration as u8,
            bConfigurationValue: 1,
            iConfiguration: 6,
            bmAttributes: 0x80, // 0b1000_0000 = bus-powered
            bMaxPower: 50,      // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 0,
                bAlternateSetting: 0,
                bInterfaceClass: 0x00,
                bInterfaceSubClass: 0x00,
                bInterfaceProtocol: 0x00,
                iInterface: 7,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    bEndpointAddress: ENDPOINT_BULK_OUT,
                    bmAttributes: 0x02, // Bulk
                    wMaxPacketSize: OTHER_DESCRIPTOR_MAX_PACKET_SIZE,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: ENDPOINT_BULK_IN,
                    bmAttributes: 0x02, // Bulk
                    wMaxPacketSize: OTHER_DESCRIPTOR_MAX_PACKET_SIZE,
                    bInterval: 0,
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
    StringDescriptor::new("0000000000000000");
pub static USB_STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("config 1");
pub static USB_STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("interface 0");
pub static USB_STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("other config 1");
pub static USB_STRING_DESCRIPTOR_7: StringDescriptor = StringDescriptor::new("other interface 0");

static USB_STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &USB_STRING_DESCRIPTOR_1,
    &USB_STRING_DESCRIPTOR_2,
    &USB_STRING_DESCRIPTOR_3,
    &USB_STRING_DESCRIPTOR_4,
    &USB_STRING_DESCRIPTOR_5,
    &USB_STRING_DESCRIPTOR_6,
    &USB_STRING_DESCRIPTOR_7,
];
