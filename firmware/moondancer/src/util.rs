use crate::hal::smolusb;
use pac::csr::interrupt;

use libgreat::GreatError;

use smolusb::event::UsbEvent;
use smolusb::setup::SetupPacket;
use smolusb::traits::{ReadControl, UnsafeUsbDriverOperations, UsbDriverOperations};

use crate::event::InterruptEvent;
use crate::{hal, pac};

use crate::debug::Bit;
use ladybug::Channel;

// - generic usb isr ----------------------------------------------------------

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::too_many_lines)]
pub fn get_usb_interrupt_event() -> InterruptEvent {
    use crate::UsbInterface::{Aux, Control, Target};

    let usb0 = unsafe { hal::Usb0::summon() }; // target
    let usb1 = unsafe { hal::Usb1::summon() }; // aux
    let usb2 = unsafe { hal::Usb2::summon() }; // control

    let pending = match interrupt::pending() {
        Ok(interrupt) => interrupt,
        Err(pending) => return InterruptEvent::UnknownInterrupt(pending),
    };

    match pending {
        // - usb0 interrupts - "target_phy" --

        // USB0 BusReset
        pac::Interrupt::USB0 => {
            ladybug::trace(Channel::A, Bit::B_IRQ_BUS_RESET, || {
                usb0.device
                    .ev_pending()
                    .modify(|r, w| w.mask().bit(r.mask().bit()));

                // handle bus reset in interrupt handler for lowest latency
                usb0.bus_reset();
                InterruptEvent::Usb(Target, UsbEvent::BusReset)
            })
        }

        // USB0_EP_CONTROL ReceiveSetupPacket
        pac::Interrupt::USB0_EP_CONTROL => {
            ladybug::trace(Channel::B, Bit::B_IRQ_EP_CONTROL, || {
                usb0.ep_control
                    .ev_pending()
                    .modify(|r, w| w.mask().bit(r.mask().bit()));

                // read setup packet in interrupt handler for lowest latency
                let endpoint_number = usb0.ep_control.status().read().epno().bits() as u8;
                let mut setup_packet_buffer = [0_u8; 8];
                let bytes_read = usb0.read_control(&mut setup_packet_buffer);
                let setup_packet = SetupPacket::from(setup_packet_buffer);
                if bytes_read == 0 {
                    InterruptEvent::ErrorMessage("ERROR USB0 received 0 bytes for setup packet")
                } else {
                    InterruptEvent::Usb(
                        Target,
                        UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet),
                    )
                }
            })
        }

        // USB0_EP_IN SendComplete
        pac::Interrupt::USB0_EP_IN => ladybug::trace(Channel::B, Bit::B_IRQ_EP_IN, || {
            usb0.ep_in
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb0.ep_in.status().read().epno().bits() as u8;
            unsafe {
                usb0.clear_tx_ack_active(endpoint_number);
            }

            InterruptEvent::Usb(Target, UsbEvent::SendComplete(endpoint_number))
        }),

        // USB0_EP_OUT ReceivePacket
        pac::Interrupt::USB0_EP_OUT => ladybug::trace(Channel::B, Bit::B_IRQ_EP_OUT, || {
            usb0.ep_out
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb0.ep_out.status().read().epno().bits() as u8;
            InterruptEvent::Usb(Target, UsbEvent::ReceivePacket(endpoint_number))
        }),

        // - usb1 interrupts - "aux_phy" (host on r0.4) --

        // USB1 BusReset
        pac::Interrupt::USB1 => {
            usb1.device
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            // handle bus reset in interrupt handler for lowest latency
            usb1.bus_reset();
            InterruptEvent::Usb(Aux, UsbEvent::BusReset)
        }

        // USB1_EP_CONTROL ReceiveSetupPacket
        pac::Interrupt::USB1_EP_CONTROL => {
            usb1.ep_control
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            // read setup packet in interrupt handler for lowest latency
            let endpoint_number = usb1.ep_control.status().read().epno().bits() as u8;
            let mut setup_packet_buffer = [0_u8; 8];
            let bytes_read = usb1.read_control(&mut setup_packet_buffer);
            let setup_packet = SetupPacket::from(setup_packet_buffer);
            if bytes_read == 0 {
                InterruptEvent::ErrorMessage("ERROR USB1 received 0 bytes for setup packet")
            } else {
                InterruptEvent::Usb(
                    Aux,
                    UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet),
                )
            }
        }

        // USB1_EP_IN SendComplete
        pac::Interrupt::USB1_EP_IN => {
            usb1.ep_in
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb1.ep_in.status().read().epno().bits() as u8;
            unsafe {
                usb1.clear_tx_ack_active(endpoint_number);
            }

            InterruptEvent::Usb(Aux, UsbEvent::SendComplete(endpoint_number))
        }

        // USB1_EP_OUT ReceivePacket}
        pac::Interrupt::USB1_EP_OUT => {
            usb1.ep_out
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb1.ep_out.status().read().epno().bits() as u8;
            InterruptEvent::Usb(Aux, UsbEvent::ReceivePacket(endpoint_number))
        }

        // - usb2 interrupts - "control_phy" (sideband on r0.4) --

        // USB2 BusReset
        pac::Interrupt::USB2 => {
            usb2.device
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            // handle bus reset in interrupt handler for lowest latency
            usb2.bus_reset();
            InterruptEvent::Usb(Control, UsbEvent::BusReset)
        }

        // USB2_EP_CONTROL ReceiveControl
        pac::Interrupt::USB2_EP_CONTROL => {
            usb2.ep_control
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            // read setup packet in interrupt handler for lowest latency
            let endpoint_number = usb2.ep_control.status().read().epno().bits() as u8;
            let mut setup_packet_buffer = [0_u8; 8];
            let bytes_read = usb2.read_control(&mut setup_packet_buffer);
            let setup_packet = SetupPacket::from(setup_packet_buffer);
            if bytes_read == 0 {
                InterruptEvent::ErrorMessage("ERROR USB1 received 0 bytes for setup packet")
            } else {
                InterruptEvent::Usb(
                    Control,
                    UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet),
                )
            }
        }

        // USB2_EP_IN SendComplete / NAK
        pac::Interrupt::USB2_EP_IN => {
            usb2.ep_in
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb2.ep_in.status().read().epno().bits() as u8;
            unsafe {
                usb2.clear_tx_ack_active(endpoint_number);
            }

            InterruptEvent::Usb(Control, UsbEvent::SendComplete(endpoint_number))
        }

        // USB2_EP_OUT ReceivePacket
        pac::Interrupt::USB2_EP_OUT => {
            usb2.ep_out
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint_number = usb2.ep_out.data().read().byte().bits() as u8;
            InterruptEvent::Usb(Control, UsbEvent::ReceivePacket(endpoint_number))
        }

        // Unhandled
        _ => InterruptEvent::UnhandledInterrupt(pending),
    }
}

// - multi event queue --------------------------------------------------------

use heapless::mpmc::MpMcQueue as Queue;

#[allow(non_snake_case)]
pub mod UsbEventExt {
    //! Alternate implementation of some [`UsbEvent`](smolusb::event::UsbEvent) values that also
    //! contain their associated data.

    use crate::hal::smolusb;
    use crate::UsbInterface;

    use smolusb::setup::SetupPacket;

    /// Received a setup packet on [`USB0_EP_CONTROL`](crate::pac::Interrupt::USB0_EP_CONTROL)
    ///
    /// An alternate version of `ReceiveControl` that can be used
    /// when the setup packet is read inside the interrupt handler
    /// for lower latency.
    ///
    /// Contents is (`usb_interface`, `endpoint_number`, `setup_packet`)
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct ReceiveControl(UsbInterface, u8, SetupPacket);

    /// Received a data packet on [`USB0_EP_OUT`](crate::pac::Interrupt::USB0_EP_OUT)
    ///
    /// An alternate version of `ReceivePacket` that can be used
    /// when the packet is read inside the interrupt handler
    /// for lower latency.
    ///
    /// Contents is (`usb_interface`, `endpoint_number`, `bytes_read`, `packet_buffer`)
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct ReceivePacket(UsbInterface, u8, usize, [u8; smolusb::EP_MAX_PACKET_SIZE]);
}

/// An event queue with separate queues for interrupt events and usb events.
///
/// So the problem this solves is that some events are much larger
/// than others.
///
/// This can create some pressure on memory-use if you need a large
/// event queue.
///
/// Fortunately the larger events occur less frequently which means we
/// can give them their own, smaller, queues.
///
/// It goes something like this:
///
///     use core::any::Any;
///     use moondancer::util::MultiEventQueue;
///
///     static EVENT_QUEUE: MultiEventQueue = MultiEventQueue::new();
///     fn dispatch_event<T: Any>(event: T) {
///         match EVENT_QUEUE.enqueue(event) {
///             Ok(()) => (),
///             Err(_) => {
///                 panic!("MachineExternal - event queue overflow");
///             }
///         }
///     }
pub struct MultiEventQueue {
    receive_control: Queue<UsbEventExt::ReceiveControl, 16>,
    receive_packet: Queue<UsbEventExt::ReceivePacket, 16>,
    interrupt_event: Queue<InterruptEvent, 64>,
}

use core::any::Any;

impl MultiEventQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            receive_control: Queue::new(),
            receive_packet: Queue::new(),
            interrupt_event: Queue::new(),
        }
    }

    pub fn dequeue(&self) -> Option<InterruptEvent> {
        self.interrupt_event.dequeue()
    }

    pub fn dequeue_setup_packet(&self) -> Option<UsbEventExt::ReceiveControl> {
        self.receive_control.dequeue()
    }

    pub fn dequeue_buffer(&self) -> Option<UsbEventExt::ReceivePacket> {
        self.receive_packet.dequeue()
    }

    /// Enqueues the given event if there is sufficient space in its corresponding queue.
    ///
    /// # Errors
    ///
    /// If the queue is full it will return the event.
    pub fn enqueue<T: Any>(&self, event: T) -> Result<(), T> {
        let any = &event as &dyn Any;

        if let Some(eventref) = any.downcast_ref::<InterruptEvent>() {
            if self.interrupt_event.enqueue(*eventref).is_err() {
                log::error!("MultiEventQueue - interrupt event queue overflow");
                return Err(event);
            }
        }

        if let Some(eventref) = any.downcast_ref::<UsbEventExt::ReceiveControl>() {
            if self.receive_control.enqueue(*eventref).is_err() {
                log::error!("MultiEventQueue - usb receive control queue overflow");
                return Err(event);
            }
        }

        if let Some(eventref) = any.downcast_ref::<UsbEventExt::ReceivePacket>() {
            if self.receive_packet.enqueue(*eventref).is_err() {
                log::error!("MultiEventQueue - usb receive packet queue overflow");
                return Err(event);
            }
        }

        Ok(())
    }
}

/// Reads Cynthion's SPI Flash UUID
pub fn read_flash_uuid(spi0: &pac::SPI0) -> Result<[u8; 8], GreatError> {
    // FIXME wait for things to settle
    unsafe {
        riscv::asm::delay(80_000_000);
    }

    // configure spi0 phy
    spi0.phy().write(|w| unsafe {
        w.length().bits(8).width().bits(1).mask().bits(1)
    });

    // chip-select
    spi0.cs().write(|w| w.select().bit(false));

    // check if we can write to spi0
    if !spi_ready(&|| spi0.status().read().tx_ready().bit()) {
        log::error!("spi write timeout");
        return Err(GreatError::StreamIoctlTimeout);
    }

    // write flash id command to spi0
    let command: [u8; 13] = [0x4b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for byte in command {
        spi0.data().write(|w| unsafe { w.tx().bits(u32::from(byte)) });
    }

    // check if we can read from spi0
    if !spi_ready(&|| spi0.status().read().rx_ready().bit()) {
        log::error!("read_flash_uuid spi read timeout");
        return Err(GreatError::StreamIoctlTimeout);
    }

    // read response
    let mut response = [0_u8; 32];
    let mut n = 0;
    while spi0.status().read().rx_ready().bit() {
        response[n] = spi0.data().read().rx().bits() as u8;
        n += 1;
        if n >= response.len() {
            log::error!("read_flash_uuid read overflow");
            return Err(GreatError::BadMessage);
        }
    }

    // check response
    if n != 13 {
        log::error!(
            "read_flash_uuid invalid response length: {} - {:02x?}",
            n,
            &response[..n]
        );
        return Err(GreatError::BadMessage);
    }

    let mut ret = [0_u8; 8];
    ret[..].copy_from_slice(&response[5..13]);

    Ok(ret)
}

/// Formats a buffer containing a flash uuid into a String
#[must_use]
pub fn format_flash_uuid(uuid: [u8; 8]) -> heapless::String<16> {
    use core::fmt::Write;

    let mut ret = heapless::String::<16>::new();

    for n in (0..8).rev() {
        let mut byte = heapless::String::<2>::new();
        write!(&mut byte, "{}", format_args!("{:01x}", uuid[n])).unwrap_or(());
        ret.push_str(byte.as_str()).unwrap_or(());
    }

    ret
}

/// Retries the provided closure until it either returns true or times out.
fn spi_ready(f: &dyn Fn() -> bool) -> bool {
    let mut timeout = 0;

    while !f() {
        timeout += 1;
        if timeout > 1000 {
            return false;
        }
    }

    true
}
