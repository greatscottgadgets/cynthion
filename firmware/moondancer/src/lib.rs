#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![no_std]

// - modules ------------------------------------------------------------------

pub mod debug;
pub mod error;
pub mod event;
pub mod gcp;
pub mod log;
pub mod macros;
pub mod panic_log;
pub mod usb;
pub mod util;

// - aliases ------------------------------------------------------------------

pub use lunasoc_hal as hal;
pub use lunasoc_pac as pac;

// - re-exports ---------------------------------------------------------------

pub use error::FirmwareError;
pub use libgreat::error::GreatResult;
pub use libgreat::firmware::BoardInformation;

// - constants ----------------------------------------------------------------

pub const SYSTEM_CLOCK_FREQUENCY: u32 = pac::clock::sysclk();

// TODO these need to be populated at runtime
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

// - types --------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum UsbInterface {
    Target = 0,  // Usb0
    Aux = 1,     // Usb1 (Host on r0.4)
    Control = 2, // Usb2 (Sideband on r0.4)
}
