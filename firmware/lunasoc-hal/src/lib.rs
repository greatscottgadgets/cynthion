#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![no_std]
#![allow(clippy::inline_always)]
#![allow(clippy::must_use_candidate)]

pub mod gpio;
pub mod serial;
pub mod timer;
#[cfg(feature = "usb")]
pub mod usb;

// export peripherals
pub use serial::{Serial0, Serial1};
pub use timer::Timer0;
#[cfg(feature = "usb")]
pub use usb::{Usb0, Usb1, Usb2};

// re-export dependencies
#[cfg(feature = "usb")]
pub use smolusb;

pub use lunasoc_pac as pac;

pub use embedded_hal as hal;
pub use embedded_hal_0 as hal_0;
pub(crate) use embedded_hal_nb as hal_nb;

pub use nb;
