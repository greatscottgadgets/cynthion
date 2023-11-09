use smolusb::event::UsbEvent;

use crate::pac;
use crate::UsbInterface;

/// InterruptEvent is used to notify the main loop of events received in the
/// `MachineExternal` interrupt handler.
#[derive(Copy, Clone)]
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
        match event {
            event => InterruptEvent::Usb(interface, event),
        }
    }
}

// - byte conversion ----------------------------------------------------------

impl core::convert::From<InterruptEvent> for [u8; 3] {
    // TODO lose magic numbers
    fn from(message: InterruptEvent) -> Self {
        use UsbEvent::*;
        match message {
            InterruptEvent::Usb(interface, event) => match event {
                BusReset => [event.into(), interface as u8, 0],
                ReceiveControl(endpoint_number) => {
                    [event.into(), interface as u8, endpoint_number]
                }
                #[cfg(feature="chonky_events")]
                ReceiveSetupPacket(endpoint_number, _setup_packet) => [event.into(), interface as u8, endpoint_number],
                ReceivePacket(endpoint_number) => [event.into(), interface as u8, endpoint_number],
                #[cfg(feature="chonky_events")]
                ReceiveBuffer(endpoint_number, _, _) => [event.into(), interface as u8, endpoint_number],
                SendComplete(endpoint_number) => [event.into(), interface as u8, endpoint_number],
            },
            _ => [0, 0, 0],
        }
    }
}

impl InterruptEvent {
    pub fn into_bytes(self) -> [u8; 3] {
        self.into()
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
