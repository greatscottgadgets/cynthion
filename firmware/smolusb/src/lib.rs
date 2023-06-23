#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![cfg_attr(not(test), no_std)]

//! Simple peripheral-level USB stack

pub mod class;
pub mod control;
pub mod descriptor;
pub mod device;
pub mod error;
pub mod traits;

pub use error::SmolError;
pub use error::SmolResult;
