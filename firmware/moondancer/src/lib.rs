#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![no_std]

// - modules ------------------------------------------------------------------

pub mod error;
pub mod gcp;
pub mod log;
pub mod macros;
pub mod panic_log;
pub mod usb;

// - aliases ------------------------------------------------------------------

pub use lunasoc_hal as hal;
pub use lunasoc_pac as pac;

// - re-exports ---------------------------------------------------------------

pub use error::FirmwareError;
pub use libgreat::error::GreatResult;
pub use libgreat::firmware::BoardInformation;

// - constants ----------------------------------------------------------------

pub const SYSTEM_CLOCK_FREQUENCY: u32 = pac::clock::sysclk();
pub const BOARD_INFORMATION: BoardInformation = BoardInformation {
    board_id: 0x10_u32.to_le_bytes(),
    version_string: "v2023.0.1\0",
    part_id: [0x30, 0xa, 0x00, 0xa0, 0x5e, 0x4f, 0x60, 0x00],
    serial_number: [
        0xea, 0x8e, 0xc5, 0x4f, 0x64, 0x74, 0x15, 0x15, 0x02, 0x02, 0x02, 0x03, 0xb3, 0xe0, 0x51,
        0xff,
    ],
};

pub const EP_MAX_ENDPOINTS: usize = 16;
pub const EP_MAX_PACKET_SIZE: usize = 512;

// - messages -----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum UsbInterface {
    Target  = 0, // Usb0
    Aux     = 1, // Usb1 (Host on r0.4)
    Control = 2, // Usb2 (Sideband on r0.4)
}

/// InterruptEvent is used to notify the main loop of events received in the
/// `MachineExternal` interrupt handler.
#[derive(Copy, Clone)]
pub enum InterruptEvent {
    // interrupts
    Event(pac::Interrupt),
    UnknownInterrupt(usize),

    // timer events
    Timer(usize),

    // usb events
    /// Received a USB bus reset
    ///
    /// Contents is (UsbInterface)
    UsbBusReset(UsbInterface),

    /// Received a SETUP packet on USBx_EP_CONTROL
    ///
    /// Contents is (UsbInterface, endpoint_number, SetupPacket)
    ///
    /// TODO lose SetupPacket
    UsbReceiveSetupPacket(UsbInterface, u8, smolusb::control::SetupPacket),

    /// Received a data packet on USBx_EP_OUT
    ///
    /// Contents is (UsbInterface, endpoint_number, bytes_read)
    ///
    /// TODO lose bytes_read
    UsbReceivePacket(UsbInterface, u8, usize),

    /// Send is complete on USBx_EP_IN
    ///
    /// Contents is (UsbInterface, endpoint_number)
    UsbSendComplete(UsbInterface, u8),

    // diagnostic events
    ErrorMessage(&'static str),
    DebugMessage(&'static str),
}

impl core::convert::From<InterruptEvent> for [u8; 3] {
    fn from(message: InterruptEvent) -> Self {
        match message {
            InterruptEvent::UsbBusReset(interface) => [10, interface as u8, 0],
            InterruptEvent::UsbReceiveSetupPacket(interface, endpoint_number, _) => [11, interface as u8, endpoint_number],
            InterruptEvent::UsbReceivePacket(interface, endpoint_number, _) => [12, interface as u8, endpoint_number],
            InterruptEvent::UsbSendComplete(interface, endpoint_number) => [13, interface as u8, endpoint_number],
            _ => [0, 0, 0]
        }
    }
}

impl InterruptEvent {
    pub fn into_bytes(self) -> [u8; 3] {
        self.into()
    }
}


impl core::fmt::Debug for InterruptEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // interrupts
            InterruptEvent::Event(interrupt) => write!(f, "HandleInterrupt({:?})", interrupt),
            InterruptEvent::UnknownInterrupt(interrupt) => {
                write!(f, "HandleUnknownInterrupt({})", interrupt)
            }

            // timer events
            InterruptEvent::Timer(n) => write!(f, "TimerEvent({})", n),

            // usb events
            InterruptEvent::UsbBusReset(interface) => {
                write!(f, "UsbBusReset({:?})", interface)
            }
            InterruptEvent::UsbReceiveSetupPacket(interface, endpoint, _setup_packet) => {
                write!(f, "UsbReceiveSetupPacket({:?}, {})", interface, endpoint)
            }
            InterruptEvent::UsbReceivePacket(interface, endpoint, bytes_read) => write!(
                f,
                "UsbReceiveData({:?}, {}, {})",
                interface, endpoint, bytes_read
            ),
            InterruptEvent::UsbSendComplete(interface, endpoint) => {
                write!(f, "UsbTransferComplete({:?}, {})", interface, endpoint)
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
