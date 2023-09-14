#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![cfg_attr(not(test), no_std)]

pub mod error;
pub mod firmware;
pub mod gcp;
//pub mod smolusb;

pub use error::GreatError;
pub use error::GreatResult;
