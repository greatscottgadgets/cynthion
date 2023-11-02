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
    //fn trace<R>(channel: Channel, bit_number: u8, f: impl FnOnce() -> R) -> R;

    fn high(&self, channel: Channel, bit_number: u8);
    fn low(&self, channel: Channel, bit_number: u8);
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
    fn low(&self, channel: Channel, bit_number: u8)  {}
}

// - global singleton ---------------------------------------------------------

static mut LADYBUG: &dyn LogicAnalyzer = &LadybugDummy;
