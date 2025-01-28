#![no_std]
#![no_main]

/// Simplified implementation of an USB ACM-Serial device.
///
/// To test:
///
///     picocom --imap crcrlf -b 115200 /dev/ttyACM0
///
use log::{error, info};

use moondancer::event::InterruptEvent;
use moondancer::UsbInterface;
use moondancer::{hal, pac};

use hal::smolusb;
use smolusb::class::acm;
use smolusb::control::Control;
use smolusb::device::{Descriptors, Speed};
use smolusb::event::UsbEvent;
use smolusb::setup::{Direction, Request, RequestType, SetupPacket};
use smolusb::traits::UnsafeUsbDriverOperations;
use smolusb::traits::{ReadControl, ReadEndpoint, UsbDriverOperations, WriteEndpoint};

use pac::csr::interrupt;

// - constants ----------------------------------------------------------------

const DEVICE_SPEED: Speed = Speed::Full;
const MAX_CONTROL_RESPONSE_SIZE: usize = 8;

// - global static state ------------------------------------------------------

use heapless::mpmc::MpMcQueue as Queue;

static EVENT_QUEUE: Queue<InterruptEvent, { smolusb::EP_MAX_ENDPOINTS }> = Queue::new();

#[inline(always)]
fn dispatch_event(event: InterruptEvent) {
    match EVENT_QUEUE.enqueue(event) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - event queue overflow");
            while let Some(interrupt_event) = EVENT_QUEUE.dequeue() {
                error!("{:?}", interrupt_event);
            }
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
extern "C" fn MachineExternal() {
    use UsbInterface::Target;

    // peripherals
    let peripherals = unsafe { pac::Peripherals::steal() };
    let leds = &peripherals.LEDS;
    let usb0 = unsafe { hal::Usb0::summon() };

    // debug
    leds.output()
        .write(|w| unsafe { w.bits(interrupt::bits_pending() as u8) });

    // get pending interrupt
    let pending = match pac::csr::interrupt::pending() {
        Ok(interrupt) => interrupt,
        Err(pending) => {
            dispatch_event(InterruptEvent::UnknownInterrupt(pending));
            return;
        }
    };

    match pending {
        // - Usb0 (Target) interrupts --
        pac::Interrupt::USB0 => {
            usb0.bus_reset();
            usb0.device
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));
        }
        pac::Interrupt::USB0_EP_CONTROL => {
            let endpoint = usb0.ep_control.status().read().epno().bits() as u8;
            let mut buffer = [0_u8; 8];
            let _bytes_read = usb0.read_control(&mut buffer);
            let setup_packet = SetupPacket::from(buffer);
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceiveSetupPacket(endpoint, setup_packet),
            ));
            usb0.ep_control
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));
        }
        pac::Interrupt::USB0_EP_IN => {
            let endpoint = usb0.ep_in.status().read().epno().bits() as u8;

            unsafe {
                usb0.clear_tx_ack_active(endpoint);
            }

            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::SendComplete(endpoint),
            ));
            usb0.ep_in
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));
        }
        pac::Interrupt::USB0_EP_OUT => {
            let endpoint = usb0.ep_out.status().read().epno().bits() as u8;
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceivePacket(endpoint),
            ));
            usb0.ep_out
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));
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
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    leds.output().write(|w| unsafe { w.bits(0x0) });

    // initialize logging
    moondancer::log::init();
    info!("logging initialized");

    // usb0: Target
    let mut usb0 = hal::Usb0::new(
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
            device_descriptor: acm::DEVICE_DESCRIPTOR,
            configuration_descriptor: acm::CONFIGURATION_DESCRIPTOR_0,
            string_descriptor_zero: acm::STRING_DESCRIPTOR_0,
            string_descriptors: acm::STRING_DESCRIPTORS,
            other_speed_configuration_descriptor: Some(acm::OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(acm::DEVICE_QUALIFIER_DESCRIPTOR),
            microsoft10: None,
        }
        .set_total_lengths(),
    );

    // disconnect device
    usb0.disconnect();
    unsafe {
        riscv::asm::delay(6_000_000);
    }

    // connect device
    usb0.connect(DEVICE_SPEED);
    let speed: Speed = usb0.device.status().read().speed().bits().into();
    info!("Connected USB0 device: {:?}", speed);

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
        usb0.enable_events();
    }

    info!("Peripherals initialized, entering main loop.");

    loop {
        if let Some(event) = EVENT_QUEUE.dequeue() {
            use smolusb::event::UsbEvent::*;
            use InterruptEvent::Usb;
            use UsbInterface::Target;

            match event {
                // Usb0 received a control event
                Usb(
                    Target,
                    event @ (BusReset
                    | ReceiveControl(0)
                    | ReceiveSetupPacket(0, _)
                    | ReceivePacket(0)
                    | SendComplete(0)),
                ) => {
                    if let Some(setup_packet) = control_usb0.dispatch_event(&usb0, event) {
                        // class requests are not handled by control
                        handle_class_request(&usb0, setup_packet);
                    }
                }

                // Handle serial data on OUT ep 0x04
                Usb(Target, ReceivePacket(4)) => {
                    let mut rx_buffer: [u8; smolusb::EP_MAX_PACKET_SIZE] =
                        [0; smolusb::EP_MAX_PACKET_SIZE];
                    let bytes_read = usb0.read(4, &mut rx_buffer);

                    // convert to uppercase
                    for b in rx_buffer[0..bytes_read].iter_mut() {
                        if 'a' as u8 <= *b && *b <= 'z' as u8 {
                            *b -= 32;
                        }
                    }

                    // echo back on IN ep 0x84
                    usb0.write(4, rx_buffer[0..bytes_read].iter().cloned());
                }

                // Handle send complete, prime OUT ep 0x04 to receive next packet
                Usb(Target, SendComplete(4)) => {
                    usb0.ep_out_prime_receive(4);
                }

                // unhandled
                _ => {
                    info!("Unhandled event: {:?}", event);
                }
            }
        }
    }
}

// - class request handler ---------------------------------------------------

fn handle_class_request<D>(usb: &D, setup_packet: SetupPacket)
where
    D: ReadControl + ReadEndpoint + WriteEndpoint + UsbDriverOperations + UnsafeUsbDriverOperations,
{
    let request_type = setup_packet.request_type();
    let request = setup_packet.request();
    let direction = setup_packet.direction();

    match (request_type, request) {
        (RequestType::Class, Request::ClassOrVendor(class_request)) => {
            use acm::serial::ClassRequest::{self, *};
            let class_request = ClassRequest::from(class_request);

            // In testing, macOS and Linux are fine will all requests being stalled; while Windows
            // seems to be happy as long as SET_LINE_CODING is implemented. We'll implement only
            // that, and stall every other handler.
            match (direction, class_request) {
                (Direction::HostToDevice, SetLineCoding) => {
                    // 32 - comes with 7 bytes of data
                    usb.ep_out_prime_receive(4);
                }
                // we can just stall the reset
                (Direction::DeviceToHost, _) => {
                    // IN
                    usb.stall_endpoint_out(0);
                }
                (Direction::HostToDevice, _) => {
                    // OUT
                    usb.stall_endpoint_in(0);
                }
            }
        }
        _ => {
            error!(
                "handle_class_request error - unhandled control request: {:?} {:?}",
                request_type, request
            );
        }
    }
}
