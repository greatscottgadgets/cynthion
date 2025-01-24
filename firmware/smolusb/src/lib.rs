#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![cfg_attr(not(test), no_std)]

//! A simple peripheral-level USB stack designed for [`luna-soc`](https://github.com/greatscottgadgets/luna-soc/) USB peripherals.

pub mod class;
pub mod control;
pub mod descriptor;
pub mod device;
pub mod error;
pub mod event;
pub mod setup;
pub mod traits;

/// USB devices can define up to 32 endpoints. 16 IN and 16 OUT.
pub const EP_MAX_ENDPOINTS: usize = 16;

/// Maximum packet size for endpoints.
pub const EP_MAX_PACKET_SIZE: usize = 512;

/// Returns the max packet size for a given device speed and endpoint number.
#[must_use]
pub fn max_packet_size(device_speed: device::Speed, endpoint_number: u8) -> usize {
    match (device_speed, endpoint_number) {
        (_, 0) => 64,
        (device::Speed::High, _) => EP_MAX_PACKET_SIZE,
        (device::Speed::Full, _) => 64,
        (device::Speed::Low, _) => 8,
        (_, _) => {
            log::warn!("Unsupported device speed: {:?}", device_speed);
            64
        }
    }
}
