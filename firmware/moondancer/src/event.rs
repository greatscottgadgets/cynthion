use smolusb::event::UsbEvent;

use crate::pac;
use crate::UsbInterface;

/// InterruptEvent is used to notify the main loop of events received in the
/// `MachineExternal` interrupt handler.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum InterruptEvent {
    // interrupt events
    Interrupt(pac::Interrupt),
    UnknownInterrupt(usize),
    UnhandledInterrupt(usize),

    // timer events
    Timer(usize),

    // usb events
    /// Received a USB event
    Usb(UsbInterface, UsbEvent),

    // diagnostic events
    ErrorMessage(&'static str),
    DebugMessage(&'static str),
}

// - smolusb::Event conversion ------------------------------------------------

impl TryFrom<InterruptEvent> for UsbEvent {
    type Error = ();

    /// Convert an `[InterruptEvent]` to a `[smolusb::Event]`
    fn try_from(event: InterruptEvent) -> Result<Self, Self::Error> {
        match event {
            InterruptEvent::Usb(_interface, event) => Ok(event),
            _ => Err(()),
        }
    }
}

impl InterruptEvent {
    /// Convert a `[smolusb::Event]` to an `[InterruptEvent]`
    pub fn from_smolusb_event(interface: UsbInterface, event: UsbEvent) -> InterruptEvent {
        InterruptEvent::Usb(interface, event)
    }
}

// - debug --------------------------------------------------------------------

impl core::fmt::Debug for InterruptEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // interrupts
            InterruptEvent::Interrupt(interrupt) => write!(f, "Event({:?})", interrupt),
            InterruptEvent::UnknownInterrupt(interrupt) => {
                write!(f, "UnknownInterrupt({})", interrupt)
            }
            InterruptEvent::UnhandledInterrupt(interrupt) => {
                write!(f, "UnhandledInterrupt({})", interrupt)
            }

            // timer events
            InterruptEvent::Timer(n) => write!(f, "Timer({})", n),

            // usb events
            InterruptEvent::Usb(interface, event) => {
                write!(f, "{:?} on {:?}", event, interface)
            }

            // misc
            InterruptEvent::ErrorMessage(message) => {
                write!(f, "ErrorMessage({})", message)
            }
            InterruptEvent::DebugMessage(message) => {
                write!(f, "DebugMessage({})", message)
            }
        }
    }
}
