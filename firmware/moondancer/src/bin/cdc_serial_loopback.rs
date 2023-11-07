#![allow(unused_imports)] // TODO
#![no_std]
#![no_main]

use log::{debug, error, info};

use smolusb::class::cdc;
use smolusb::control_new::{Control, Descriptors};
use smolusb::descriptor::DescriptorType;
use smolusb::device::Speed;
use smolusb::event::UsbEvent;
use smolusb::setup::{Request, RequestType, SetupPacket};
use smolusb::traits::{
    ReadControl, ReadEndpoint, UsbDriverOperations, WriteEndpoint, WriteRefEndpoint,
};

use moondancer::{hal, pac};
use pac::csr::interrupt;

// - constants ----------------------------------------------------------------

const DEVICE_SPEED: Speed = Speed::High;
const MAX_CONTROL_RESPONSE_SIZE: usize = 8;

// - types --------------------------------------------------------------------

/// The UsbDataPacket struct represents a single packet of data
/// received from a USB port.
pub struct UsbDataPacket {
    pub interface: moondancer::UsbInterface,
    pub endpoint: u8,
    pub bytes_read: usize,
    pub buffer: [u8; moondancer::EP_MAX_PACKET_SIZE],
}

// - global static state ------------------------------------------------------

use heapless::mpmc::MpMcQueue as Queue;
use moondancer::event::InterruptEvent;

static EVENT_QUEUE: Queue<InterruptEvent, { moondancer::EP_MAX_ENDPOINTS }> = Queue::new();
static USB_RECEIVE_PACKET_QUEUE: Queue<UsbDataPacket, { moondancer::EP_MAX_ENDPOINTS }> =
    Queue::new();

#[inline(always)]
fn dispatch_event(event: InterruptEvent) {
    match EVENT_QUEUE.enqueue(event) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - event queue overflow");
        }
    }
}

#[inline(always)]
fn dispatch_receive_packet(usb_receive_packet: UsbDataPacket) {
    match USB_RECEIVE_PACKET_QUEUE.enqueue(usb_receive_packet) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - usb receive packet queue overflow");
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
    let leds = &peripherals.LEDS;
    let usb0 = unsafe { hal::Usb0::summon() };
    let usb1 = unsafe { hal::Usb1::summon() };

    // debug
    let pending = interrupt::reg_pending();
    leds.output
        .write(|w| unsafe { w.output().bits(pending as u8) });

    // - Usb0 (Target) interrupts --
    if usb0.is_pending(pac::Interrupt::USB0) {
        usb0.clear_pending(pac::Interrupt::USB0);
        usb0.bus_reset();
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        let endpoint = usb0.ep_control.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
        dispatch_event(InterruptEvent::Usb(
            Target,
            UsbEvent::ReceiveControl(endpoint),
        ));
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        let endpoint = usb0.ep_in.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_IN);
        dispatch_event(InterruptEvent::Usb(
            Target,
            UsbEvent::SendComplete(endpoint)
        ));
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        // read data from endpoint
        let endpoint = usb0.ep_out.data_ep.read().bits() as u8;
        let mut receive_packet = UsbDataPacket {
            interface: Target,
            endpoint,
            bytes_read: 0,
            buffer: [0_u8; moondancer::EP_MAX_PACKET_SIZE],
        };
        receive_packet.bytes_read = usb0.read(endpoint, &mut receive_packet.buffer);

        // clear pending IRQ after data is read
        usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);

        // dispatch packet to main loop
        dispatch_receive_packet(receive_packet);

    // - Usb1 (Aux) interrupts --
    } else if usb1.is_pending(pac::Interrupt::USB1) {
        usb1.clear_pending(pac::Interrupt::USB1);
        usb1.bus_reset();
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        let endpoint = usb1.ep_control.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        dispatch_event(InterruptEvent::Usb(Aux, UsbEvent::ReceiveControl(endpoint)));
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        let endpoint = usb1.ep_in.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_IN);
        dispatch_event(InterruptEvent::Usb(
            Aux,
            UsbEvent::SendComplete(endpoint)
        ));
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        // read data from endpoint
        let endpoint = usb1.ep_out.data_ep.read().bits() as u8;
        let mut receive_packet = UsbDataPacket {
            interface: Aux,
            endpoint,
            bytes_read: 0,
            buffer: [0_u8; moondancer::EP_MAX_PACKET_SIZE],
        };
        receive_packet.bytes_read = usb1.read(endpoint, &mut receive_packet.buffer);

        // clear pending IRQ after data is read
        usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);

        // dispatch packet to main loop
        dispatch_receive_packet(receive_packet);

    // - Unknown Interrupt --
    } else {
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
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    leds.output.write(|w| unsafe { w.output().bits(0x0) });

    // initialize logging
    let serial = hal::Serial::new(peripherals.UART);
    moondancer::log::init(serial);
    info!("logging initialized");

    // usb0: Target
    let usb0 = hal::Usb0::new(
        peripherals.USB0,
        peripherals.USB0_EP_CONTROL,
        peripherals.USB0_EP_IN,
        peripherals.USB0_EP_OUT,
    );

    // usb0 control endpoint
    let mut control_usb0 = Control::<_, MAX_CONTROL_RESPONSE_SIZE>::new(
        0,
        Descriptors {
            device_speed: DEVICE_SPEED,
            device_descriptor: cdc::DEVICE_DESCRIPTOR,
            configuration_descriptor: cdc::CONFIGURATION_DESCRIPTOR_0,
            other_speed_configuration_descriptor: Some(cdc::OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(cdc::DEVICE_QUALIFIER_DESCRIPTOR),
            string_descriptor_zero: cdc::STRING_DESCRIPTOR_0,
            string_descriptors: cdc::STRING_DESCRIPTORS,
        }
        .set_total_lengths(), // TODO figure out a better solution
    );
    // TODO control_usb0.cb_vendor_request = Some(handle_vendor_request);
    // TODO control_usb0.cb_string_request = Some(handle_string_request);

    // set device speed
    usb0.set_speed(DEVICE_SPEED);

    // connect device
    usb0.connect();
    let speed: Speed = usb0.controller.speed.read().speed().bits().into();
    info!("Connected USB0 device: {:?}", speed);

    // usb1: Aux
    let usb1 = hal::Usb1::new(
        peripherals.USB1,
        peripherals.USB1_EP_CONTROL,
        peripherals.USB1_EP_IN,
        peripherals.USB1_EP_OUT,
    );

    // usb1 control endpoint
    let mut control_usb1 = Control::<_, MAX_CONTROL_RESPONSE_SIZE>::new(
        0,
        Descriptors {
            device_speed: DEVICE_SPEED,
            device_descriptor: cdc::DEVICE_DESCRIPTOR,
            configuration_descriptor: cdc::CONFIGURATION_DESCRIPTOR_0,
            other_speed_configuration_descriptor: Some(cdc::OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(cdc::DEVICE_QUALIFIER_DESCRIPTOR),
            string_descriptor_zero: cdc::STRING_DESCRIPTOR_0,
            string_descriptors: cdc::STRING_DESCRIPTORS,
        }
        .set_total_lengths(), // TODO figure out a better solution
    );
    // TODO control_usb1.cb_vendor_request = Some(handle_vendor_request);
    // TODO control_usb1.cb_string_request = Some(handle_string_request);

    // set device speed
    usb1.set_speed(DEVICE_SPEED);

    // connect device
    usb1.connect();
    let speed: Speed = usb1.controller.speed.read().speed().bits().into();
    info!("Connected USB1 device: {:?}", speed);

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable usb0 interrupts and events
        interrupt::enable(pac::Interrupt::USB0);
        interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB0_EP_IN);
        interrupt::enable(pac::Interrupt::USB0_EP_OUT);
        interrupt::enable(pac::Interrupt::USB1);
        interrupt::enable(pac::Interrupt::USB1_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB1_EP_IN);
        interrupt::enable(pac::Interrupt::USB1_EP_OUT);
        usb0.enable_interrupts();
        usb1.enable_interrupts();
    }

    // prime the usb OUT endpoints we'll be using
    usb0.ep_out_prime_receive(0);
    usb0.ep_out_prime_receive(1);
    usb0.ep_out_prime_receive(2);
    usb1.ep_out_prime_receive(0);
    usb1.ep_out_prime_receive(1);
    usb1.ep_out_prime_receive(2);

    info!("Peripherals initialized, entering main loop.");

    loop {
        if let Some(event) = EVENT_QUEUE.dequeue() {
            use moondancer::event::InterruptEvent::Usb;
            use moondancer::UsbInterface::{Aux, Target};
            use smolusb::event::UsbEvent::*;

            match event {
                // Usb0 received a control event
                Usb(Target, event @ BusReset)
                | Usb(Target, event @ ReceiveControl(0))
                | Usb(Target, event @ ReceivePacket(0))
                | Usb(Target, event @ SendComplete(0)) => {
                    match control_usb0.handle_event(&usb0, event) {
                        // vendor requests are not handled by control
                        Some((setup_packet, _rx_buffer)) => {
                            handle_vendor_request(&usb0, setup_packet);
                        }
                        // control event was handled
                        None => (),
                    }
                }

                // Usb1 received a control event
                Usb(Aux, event @ BusReset)
                | Usb(Aux, event @ ReceiveControl(0))
                | Usb(Aux, event @ ReceivePacket(0))
                | Usb(Aux, event @ SendComplete(0)) => {
                    match control_usb1.handle_event(&usb1, event) {
                        // vendor requests are not handled by control
                        Some((setup_packet, _rx_buffer)) => {
                            handle_vendor_request(&usb1, setup_packet);
                        }
                        // control event was handled
                        None => (),
                    }
                }

                // unhandled
                _ => {
                    info!("Unhandled event: {:?}", event);
                },
            }
        }

        if let Some(UsbDataPacket {
            interface,
            endpoint,
            bytes_read,
            buffer,
        }) = USB_RECEIVE_PACKET_QUEUE.dequeue()
        {
            use moondancer::UsbInterface::{Aux, Target};

            match (interface, endpoint, bytes_read, buffer) {
                // usb0 control endpoint receive packet
                (Target, 0, _bytes_read, _buffer) => {
                    control_usb0.handle_event(&usb0, UsbEvent::ReceivePacket(0));
                }

                // usb1 control endpoint receive packet
                (Aux, 0, _bytes_read, _buffer) => {
                    control_usb1.handle_event(&usb1, UsbEvent::ReceivePacket(0));
                }

                // usb0 receive packet handler
                (Target, endpoint, bytes_read, buffer) => {
                    if endpoint != 0 {
                        debug!(
                            "Received {} bytes on usb0 endpoint: {} - {:?}",
                            bytes_read,
                            endpoint,
                            &buffer[0..8],
                        );
                        usb1.write_ref(endpoint, buffer.iter().take(bytes_read).into_iter());
                        info!("Sent {} bytes to usb1 endpoint: {}", bytes_read, endpoint);
                    }
                    usb0.ep_out_prime_receive(endpoint);
                }

                // usb1 receive packet handler
                (Aux, endpoint, bytes_read, buffer) => {
                    if endpoint != 0 {
                        debug!(
                            "Received {} bytes on usb1 endpoint: {} - {:?}",
                            bytes_read,
                            endpoint,
                            &buffer[0..8],
                        );
                        usb0.write_ref(endpoint, buffer.iter().take(bytes_read).into_iter());
                        info!("Sent {} bytes to usb0 endpoint: {}", bytes_read, endpoint);
                    }
                    usb1.ep_out_prime_receive(endpoint);
                }

                // unhandled
                _ => (),
            }
        }
    }
}

// - vendor request handlers --------------------------------------------------


fn handle_vendor_request<'a, D>(
    usb: &D,
    setup_packet: SetupPacket,
) where
    D: ReadControl + ReadEndpoint + WriteEndpoint + WriteRefEndpoint + UsbDriverOperations,
{
    let request_type = setup_packet.request_type();
    let request = setup_packet.request();

    match (request_type, request) {
        (RequestType::Vendor, Request::ClassOrVendor(vendor_request)) => {
            let vendor_request =
                cdc::ch34x::VendorRequest::from(vendor_request);
            info!("CDC-SERIAL vendor request: {:?} {} {}", vendor_request, setup_packet.value, setup_packet.index);

            // we can just spoof these
            usb.write(0, [0, 0].into_iter());
        }
        (RequestType::Standard, Request::GetDescriptor) => {
            let [index, descriptor_type_bits] = setup_packet.value.to_le_bytes();
            match DescriptorType::try_from(descriptor_type_bits) {
                Ok(DescriptorType::String) => {
                    debug!("CDC-SERIAL string_request: {}", index);

                    // we can just spoof this too
                    usb.write(0, [].into_iter());
                }
                _ => {
                    error!(
                        "handle_vendor_request error - unhandled descriptor request: {:?} {:?}",
                        request_type, request
                    );
                }
            }
        }
        _ => {
            error!(
                "handle_vendor_request error - unhandled control request: {:?} {:?}",
                request_type, request
            );
        }
    }
}
