//! ladybug implementation for cynthion
use crate::pac;

#[non_exhaustive]
pub struct Bit;

impl Bit {
    // - PMOD A --

    pub const A_GET_EVENTS: u8 = 0;
    pub const A_READ_CONTROL: u8 = 1;
    pub const A_READ_ENDPOINT: u8 = 2;
    pub const A_WRITE_ENDPOINT: u8 = 3;
    pub const A_PRIME_RECEIVE: u8 = 4;

    pub const A_PACKET_PUSH: u8 = 6;
    pub const A_PACKET_POP: u8 = 7;

    // - PMOD B --

    pub const B_EP_IS_0: u8 = 0;
    pub const B_EP_IS_1: u8 = 1;
    pub const B_IRQ_BUS_RESET: u8 = 2;
    pub const B_IRQ_EP_CONTROL: u8 = 3;
    pub const B_IRQ_EP_IN: u8 = 4;
    pub const B_IRQ_EP_OUT: u8 = 5;
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_panics_doc)]
pub fn init(_gpioa: pac::GPIO0, _gpiob: pac::GPIO1) {
    #[cfg(feature = "ladybug")]
    unsafe {
        use crate::debug::ladybug_impl::{LadybugCynthion, LADYBUG_CYNTHION};
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
        #[allow(clippy::similar_names)]
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
                _ => {}
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
                _ => {}
            }
        }
    }
}
