use core::sync::atomic::{AtomicU8, Ordering};

use ladybug::{Channel, LogicAnalyzer};

use crate::pac;


// - ladybug helpers ----------------------------------------------------------

pub fn init() {
    unsafe { ladybug::set_analyzer(&LADYBUG_CYNTHION); }
}

// - global singleton for ladybug cynthion implementation ---------------------

static LADYBUG_CYNTHION: LadybugCynthion = LadybugCynthion::new();

struct LadybugCynthion {
    a: AtomicU8,
    b: AtomicU8,
}

impl LadybugCynthion {
    pub const fn new() -> Self {
        Self {
            a: AtomicU8::new(0),
            b: AtomicU8::new(0),
        }
    }
}

impl LogicAnalyzer for LadybugCynthion {
    fn high(&self, channel: Channel, bit_number: u8) {
        match channel {
            Channel::A => {
                let gpioa = unsafe { &pac::Peripherals::steal().GPIOA };
                gpioa.odr.write(|w| unsafe {
                    self.a.fetch_or(1 << bit_number, Ordering::Relaxed);
                    w.odr().bits(self.a.load(Ordering::Relaxed))
                });
            }
            Channel::B => {
                let gpiob = unsafe { &pac::Peripherals::steal().GPIOB };
                gpiob.odr.write(|w| unsafe {
                    self.b.fetch_or(1 << bit_number, Ordering::Relaxed);
                    w.odr().bits(self.b.load(Ordering::Relaxed))
                });
            }
        }
    }

    fn low(&self, channel: Channel, bit_number: u8) {
        match channel {
            Channel::A => {
                let gpioa = unsafe { &pac::Peripherals::steal().GPIOA };
                gpioa.odr.write(|w| unsafe {
                    self.a.fetch_xor(1 << bit_number, Ordering::Relaxed);
                    w.odr().bits(self.a.load(Ordering::Relaxed))
                });
            }
            Channel::B => {
                let gpiob = unsafe { &pac::Peripherals::steal().GPIOB };
                gpiob.odr.write(|w| unsafe {
                    self.b.fetch_xor(1 << bit_number, Ordering::Relaxed);
                    w.odr().bits(self.b.load(Ordering::Relaxed))
                });
            }
        }
    }
}
