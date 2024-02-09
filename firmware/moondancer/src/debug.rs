//! ladybug implementation for cynthion
use crate::pac;

#[allow(clippy::needless_pass_by_value)]
pub fn init(_gpioa: pac::GPIOA, _gpiob: pac::GPIOB) {
    #[cfg(feature = "ladybug")]
    unsafe {
        use crate::debug::ladybug_impl::*;
        LADYBUG_CYNTHION = Some(LadybugCynthion::new(_gpioa, _gpiob));
        ladybug::set_analyzer(LADYBUG_CYNTHION.as_ref().expect("surprises"));
    }
}

#[cfg(feature = "ladybug")]
mod ladybug_impl {
    use crate::pac;
    use core::sync::atomic::{AtomicU8, Ordering};
    use ladybug::{Channel, LogicAnalyzer};

    pub static mut LADYBUG_CYNTHION: Option<LadybugCynthion> = None;

    #[cfg(feature = "ladybug")]
    pub struct LadybugCynthion {
        gpioa: pac::GPIOA,
        gpiob: pac::GPIOB,
        a: AtomicU8,
        b: AtomicU8,
    }

    impl LadybugCynthion {
        pub fn new(gpioa: pac::GPIOA, gpiob: pac::GPIOB) -> Self {
            // configure gpioa & gpiob pins as outputs
            gpioa
                .moder()
                .write(|w| unsafe { w.moder().bits(0b1111_1111) }); // 0=input, 1=output
            gpiob
                .moder()
                .write(|w| unsafe { w.moder().bits(0b1111_1111) }); // 0=input, 1=output

            Self {
                gpioa,
                gpiob,
                a: AtomicU8::new(0),
                b: AtomicU8::new(0),
            }
        }
    }

    impl LogicAnalyzer for LadybugCynthion {
        fn high(&self, channel: Channel, bit_number: u8) {
            match channel {
                Channel::A => {
                    self.gpioa.odr().write(|w| unsafe {
                        self.a.fetch_or(1 << bit_number, Ordering::Relaxed);
                        w.odr().bits(self.a.load(Ordering::Relaxed))
                    });
                }
                Channel::B => {
                    self.gpiob.odr().write(|w| unsafe {
                        self.b.fetch_or(1 << bit_number, Ordering::Relaxed);
                        w.odr().bits(self.b.load(Ordering::Relaxed))
                    });
                }
            }
        }

        fn low(&self, channel: Channel, bit_number: u8) {
            match channel {
                Channel::A => {
                    self.gpioa.odr().write(|w| unsafe {
                        self.a.fetch_xor(1 << bit_number, Ordering::Relaxed);
                        w.odr().bits(self.a.load(Ordering::Relaxed))
                    });
                }
                Channel::B => {
                    self.gpiob.odr().write(|w| unsafe {
                        self.b.fetch_xor(1 << bit_number, Ordering::Relaxed);
                        w.odr().bits(self.b.load(Ordering::Relaxed))
                    });
                }
            }
        }
    }
}
