#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![cfg_attr(not(test), no_std)]

use core::cell::RefCell;
use core::marker::PhantomData;

// - public types -------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Channel {
    A,
    B,
}

pub trait LogicAnalyzer {
    fn high(&self, channel: Channel, bit_number: u8);
    fn low(&self, channel: Channel, bit_number: u8);
}

#[non_exhaustive]
pub struct Bit;

impl Bit {
    // - PMOD A --
    // usb_hal.rs
    pub const A_HANDLE_EVENT: u8 = 0;
    pub const A_HANDLE_VENDOR: u8 = 1;

    // moondancer.rs:gcp
    pub const A_READ_ENDPOINT: u8 = 0;
    pub const A_WRITE_ENDPOINT: u8 = 1;

    // moondancer.rs:bin
    pub const A_GCP_DISPATCH_REQUEST: u8 = 2;
    pub const A_GCP_DISPATCH_RESPONSE: u8 = 5;
    pub const A_GCP_DISPATCH_ABORT: u8 = 6;

    // usb.rs
    pub const A_USB_STALL_IN: u8 = 2;
    pub const A_USB_STALL_OUT: u8 = 2;
    pub const A_USB_READ: u8 = 3;
    pub const A_USB_WRITE: u8 = 4;
    pub const A_USB_RX_ZLP: u8 = 5;
    pub const A_USB_TX_ZLP: u8 = 5;
    pub const A_USB_ACK: u8 = 6;
    pub const A_USB_EP_OUT_PRIME: u8 = 7;

    // - PMOD B --

    // interrupts
    pub const B_IRQ_BUS_RESET: u8 = 0;
    pub const B_IRQ_EP_CONTROL: u8 = 1;
    pub const B_IRQ_EP_IN: u8 = 2;
    pub const B_IRQ_EP_OUT: u8 = 3;

    // usb.rs
    pub const B_USB_READ_CONTROL: u8 = 4;
    pub const B_USB_EP_IN_EPNO: u8 = 5;

    // pmod B 6, 7 are not used because LA channels are assigned to USB D+/D1
}

// - public methods -----------------------------------------------------------

pub unsafe fn set_analyzer(analyzer: &'static dyn LogicAnalyzer) {
    LADYBUG = analyzer;
}

/// Returns a reference to the logic analyzer.
pub fn ladybug() -> &'static dyn LogicAnalyzer {
    unsafe { LADYBUG }
}

#[inline(always)]
pub fn trace<R>(channel: Channel, bit_number: u8, f: impl FnOnce() -> R) -> R {
    #[cfg(not(feature = "enable"))]
    {
        f()
    }
    #[cfg(feature = "enable")]
    {
        ladybug().high(channel, bit_number);
        let result = f();
        ladybug().low(channel, bit_number);
        result
    }
}

// - No-op LogicAnalyzer ------------------------------------------------------

struct LadybugDummy;
impl LogicAnalyzer for LadybugDummy {
    fn high(&self, channel: Channel, bit_number: u8) {}
    fn low(&self, channel: Channel, bit_number: u8) {}
}

// - global singleton ---------------------------------------------------------

static mut LADYBUG: &dyn LogicAnalyzer = &LadybugDummy;
