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
    // interrupts on pmod B
    pub const IRQ_BUS_RESET: u8 = 0;
    pub const IRQ_EP_CONTROL: u8 = 1;
    pub const IRQ_EP_IN: u8 = 2;
    pub const IRQ_EP_OUT: u8 = 3;

    // moondancer
    //pub const MD_HANDLE_EVENT: u8 = 0;
    //pub const MD_HANDLE_VENDOR: u8 = 1;

    // gcp
    pub const GCP_HANDLE_EVENT: u8 = 0;
    pub const USB_ZERO_SETUP: u8   = 1;

    // control
    //pub const CONTROL_CALLBACK: u8 = 2;

    // usb
    pub const USB_STALL_IN: u8 = 2;
    pub const USB_STALL_OUT: u8 = 2;
    pub const USB_READ: u8 = 3;
    pub const USB_WRITE: u8 = 4;
    pub const USB_RX_ZLP: u8 = 5;
    pub const USB_TX_ZLP: u8 = 5;
    pub const USB_ACK: u8 = 6;
    pub const USB_EP_OUT_PRIME: u8 = 7;

    // extra events on pmod B
    pub const B_USB_READ_CONTROL: u8 = 4;
    pub const B_USB_: u8 = 5;

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
