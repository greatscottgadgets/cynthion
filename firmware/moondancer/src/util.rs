use ladybug::{Bit, Channel};

use smolusb::event::UsbEvent;
use smolusb::setup::SetupPacket;
use smolusb::traits::{ReadControl, UnsafeUsbDriverOperations, UsbDriverOperations};

use crate::event::InterruptEvent;
use crate::{hal, pac};

use pac::csr::interrupt;

// - generic usb isr ----------------------------------------------------------

pub fn get_usb_interrupt_event() -> InterruptEvent {
    use crate::UsbInterface::{Aux, Control, Target};

    let usb0 = unsafe { hal::Usb0::summon() }; // target
    let usb1 = unsafe { hal::Usb1::summon() }; // aux
    let usb2 = unsafe { hal::Usb2::summon() }; // control

    let pending = interrupt::reg_pending();

    // - usb0 interrupts - "target_phy" --

    // USB0 BusReset
    if usb0.is_pending(pac::Interrupt::USB0) {
        ladybug::trace(Channel::B, Bit::B_IRQ_BUS_RESET, || {
            usb0.clear_pending(pac::Interrupt::USB0);
            // handle bus reset in interrupt handler for lowest latency
            usb0.bus_reset();
            InterruptEvent::Usb(Target, UsbEvent::BusReset)
        })

    // USB0_EP_CONTROL ReceiveControl
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        ladybug::trace(Channel::B, Bit::B_IRQ_EP_CONTROL, || {
            let endpoint_number = usb0.ep_control.epno().read().bits() as u8;

            // read setup packet in interrupt handler for lowest latency
            let mut setup_packet_buffer = [0_u8; 8];
            let bytes_read = usb0.read_control(&mut setup_packet_buffer);
            let setup_packet = SetupPacket::from(setup_packet_buffer);
            usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
            if bytes_read == 0 {
                InterruptEvent::ErrorMessage("ERROR USB0 received 0 bytes for setup packet")
            } else {
                InterruptEvent::Usb(
                    Target,
                    UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet),
                )
            }
        })

    // USB0_EP_OUT ReceivePacket
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        ladybug::trace(Channel::B, Bit::B_IRQ_EP_OUT, || {
            let endpoint_number = usb0.ep_out.data_ep().read().bits() as u8;

            usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);
            InterruptEvent::Usb(Target, UsbEvent::ReceivePacket(endpoint_number))
        })

    // USB0_EP_IN SendComplete
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        ladybug::trace(Channel::B, Bit::B_IRQ_EP_IN, || {
            let endpoint_number = usb0.ep_in.epno().read().bits() as u8;
            usb0.clear_pending(pac::Interrupt::USB0_EP_IN);

            unsafe {
                usb0.clear_tx_ack_active(endpoint_number);
            }

            InterruptEvent::Usb(Target, UsbEvent::SendComplete(endpoint_number))
        })

    // - usb1 interrupts - "aux_phy" (host on r0.4) --

    // USB1 BusReset
    } else if usb1.is_pending(pac::Interrupt::USB1) {
        usb1.clear_pending(pac::Interrupt::USB1);
        // handle bus reset in interrupt handler for lowest latency
        usb1.bus_reset();
        InterruptEvent::Usb(Aux, UsbEvent::BusReset)

    // USB1_EP_CONTROL ReceiveControl
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        let endpoint_number = usb1.ep_control.epno().read().bits() as u8;

        // read setup packet in interrupt handler for lowest latency
        let mut setup_packet_buffer = [0_u8; 8];
        let bytes_read = usb1.read_control(&mut setup_packet_buffer);
        let setup_packet = SetupPacket::from(setup_packet_buffer);
        usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        if bytes_read == 0 {
            InterruptEvent::ErrorMessage("ERROR USB1 received 0 bytes for setup packet")
        } else {
            InterruptEvent::Usb(
                Aux,
                UsbEvent::ReceiveSetupPacket(endpoint_number, setup_packet),
            )
        }

    // USB1_EP_OUT ReceivePacket
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        let endpoint_number = usb1.ep_out.data_ep().read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);
        InterruptEvent::Usb(Aux, UsbEvent::ReceivePacket(endpoint_number))

    // USB1_EP_IN SendComplete
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        let endpoint_number = usb1.ep_in.epno().read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_IN);

        unsafe {
            usb1.clear_tx_ack_active(endpoint_number);
        }

        InterruptEvent::Usb(Aux, UsbEvent::SendComplete(endpoint_number))

    // - usb2 interrupts - "control_phy" (sideband on r0.4) --

    // USB2 BusReset
    } else if usb2.is_pending(pac::Interrupt::USB2) {
        usb2.clear_pending(pac::Interrupt::USB2);
        // handle bus reset in interrupt handler for lowest latency
        usb2.bus_reset();
        InterruptEvent::Usb(Control, UsbEvent::BusReset)

    // USB2_EP_CONTROL ReceiveControl
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_CONTROL) {
        let endpoint_number = usb2.ep_control.epno().read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_CONTROL);
        InterruptEvent::Usb(Control, UsbEvent::ReceiveControl(endpoint_number))

    // USB2_EP_OUT ReceivePacket
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_OUT) {
        let endpoint_number = usb2.ep_out.data_ep().read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_OUT);
        InterruptEvent::Usb(Control, UsbEvent::ReceivePacket(endpoint_number))

    // USB2_EP_IN SendComplete / NAK
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_IN) {
        let endpoint_number = usb2.ep_in.epno().read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_IN);

        unsafe {
            usb2.clear_tx_ack_active(endpoint_number);
        }

        InterruptEvent::Usb(Control, UsbEvent::SendComplete(endpoint_number))

    // - unhandled interrupt --
    } else {
        InterruptEvent::UnhandledInterrupt(pending)
    }
}

// - multi event queue --------------------------------------------------------

use heapless::mpmc::MpMcQueue as Queue;

#[allow(non_snake_case)]
pub mod UsbEventExt {
    //! Alternate implementation of some UsbEvent values that also
    //! contain their associated data.

    use crate::UsbInterface;
    use smolusb::setup::SetupPacket;

    /// Received a setup packet on USBx_EP_CONTROL
    ///
    /// An alternate version of `ReceiveControl` that can be used
    /// when the setup packet is read inside the interrupt handler
    /// for lower latency.
    ///
    /// Contents is (usb_interface, endpoint_number, setup_packet)
    #[derive(Clone, Copy)]
    pub struct ReceiveControl(UsbInterface, u8, SetupPacket);

    /// Received a data packet on USBx_EP_OUT
    ///
    /// An alternate version of `ReceivePacket` that can be used
    /// when the packet is read inside the interrupt handler
    /// for lower latency.
    ///
    /// Contents is (usb_interface, endpoint_number, bytes_read, packet_buffer)
    #[derive(Clone, Copy)]
    pub struct ReceivePacket(UsbInterface, u8, usize, [u8; smolusb::EP_MAX_PACKET_SIZE]);
}

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
///     use moondancer::util::EventQueue;
///
///     static EVENT_QUEUE_ALT: EventQueue = EventQueue::new();
///     fn dispatch_event_alt<T: Any>(event: T) {
///         match EVENT_QUEUE_ALT.enqueue(event) {
///             Ok(()) => (),
///             Err(_) => {
///                 error!("MachineExternal - event queue overflow");
///                 panic!("MachineExternal - event queue overflow");
///             }
///         }
///     }
pub struct EventQueue {
    receive_control: Queue<UsbEventExt::ReceiveControl, 16>,
    receive_packet: Queue<UsbEventExt::ReceivePacket, 16>,
    interrupt_event: Queue<InterruptEvent, 64>,
}

use core::any::Any;

impl EventQueue {
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

    pub fn enqueue<T: Any>(&self, event: T) -> Result<(), ()> {
        let any = &event as &dyn Any;
        match any.downcast_ref::<InterruptEvent>() {
            Some(event) => match self.interrupt_event.enqueue(*event) {
                Ok(()) => (),
                Err(_) => {
                    log::error!("EventQueue - interrupt event queue overflow");
                }
            },
            None => {}
        }

        match any.downcast_ref::<UsbEventExt::ReceiveControl>() {
            Some(event) => match self.receive_control.enqueue(*event) {
                Ok(()) => (),
                Err(_) => {
                    log::error!("EventQueue - usb receive control queue overflow");
                }
            },
            None => {}
        }

        match any.downcast_ref::<UsbEventExt::ReceivePacket>() {
            Some(event) => match self.receive_packet.enqueue(*event) {
                Ok(()) => (),
                Err(_) => {
                    log::error!("EventQueue - usb receive packet queue overflow");
                }
            },
            None => {}
        }

        Ok(())
    }
}
