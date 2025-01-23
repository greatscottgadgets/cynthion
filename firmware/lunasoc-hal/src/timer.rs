/// Timer Events
///
/// Each event is a possible interrupt source, if enabled.
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

pub enum Mode {
    OneShot,
    Periodic,
}

#[macro_export]
macro_rules! impl_timer {
    ($(
        $TIMERX:ident: $PACTIMERX:ty,
    )+) => {
        $(
            /// Timer peripheral
            #[derive(Debug)]
            pub struct $TIMERX {
                registers: $PACTIMERX,
                /// System clock speed.
                pub clk: u32,
            }

            // lifecycle
            impl $TIMERX {
                /// Create a new `Timer` from the [`TIMER`](crate::pac::TIMER) peripheral.
                pub fn new(registers: $PACTIMERX, clk: u32) -> Self {
                    Self { registers, clk }
                }

                /// Release the [`TIMER`](crate::pac::TIMER) peripheral and consume self.
                pub fn free(self) -> $PACTIMERX {
                    self.registers
                }

                /// Obtain a static `Timer` instance for use in e.g. interrupt handlers
                ///
                /// # Safety
                ///
                /// 'Tis thine responsibility, that which thou doth summon.
                pub unsafe fn summon() -> Self {
                    Self {
                        registers: <$PACTIMERX>::steal(),
                        clk: 0,
                    }
                }
            }

            // configuration
            impl $TIMERX {
                /// Current timer count
                pub fn counter(&self) -> u32 {
                    self.registers.counter().read().value().bits()
                }

                /// Disable timer
                pub fn disable(&self) {
                    self.registers.enable().write(|w| w.enable().bit(false));
                }

                /// Enable timer
                pub fn enable(&self) {
                    self.registers.enable().write(|w| w.enable().bit(true));
                }

                /// Set timeout using a [`core::time::Duration`]
                pub fn set_timeout<T>(&mut self, timeout: T)
                where
                    T: Into<core::time::Duration>
                {
                    const NANOS_PER_SECOND: u64 = 1_000_000_000;
                    let timeout = timeout.into();

                    let clk = self.clk as u64;
                    let ticks = u32::try_from(
                        clk * timeout.as_secs() +
                        clk * u64::from(timeout.subsec_nanos()) / NANOS_PER_SECOND,
                    ).unwrap_or(u32::max_value());

                    self.set_timeout_ticks(ticks.max(1));
                }

                /// Set timer mode
                pub fn set_mode(&mut self, mode: $crate::timer::Mode) {
                    let mode = match mode {
                        $crate::timer::Mode::OneShot  => false,
                        $crate::timer::Mode::Periodic => true,
                    };
                    self.registers.mode().write(|w| unsafe {
                        w.periodic().bit(mode)
                    });

                }

                /// Set timeout using system ticks
                pub fn set_timeout_ticks(&mut self, ticks: u32) {
                    self.registers.reload().write(|w| unsafe {
                        w.value().bits(ticks)
                    });
                }
            }

            // interrupts
            impl $TIMERX {

                /// Start listening for [`Event`]
                pub fn listen(&mut self, event: $crate::timer::Event) {
                    match event {
                        $crate::timer::Event::TimeOut => {
                            self.registers.ev_enable().write(|w| unsafe { w.mask().bit(true) });
                        }
                    }
                }

                /// Stop listening for [`Event`]
                pub fn unlisten(&mut self, event: $crate::timer::Event) {
                    match event {
                        $crate::timer::Event::TimeOut => {
                            self.registers.ev_enable().write(|w| unsafe { w.mask().bit(false) });
                        }
                    }
                }

                /// Check if the interrupt flag is pending
                pub fn is_pending(&self) -> bool {
                    self.registers.ev_pending().read().mask().bit()
                }

                /// Clear the interrupt flag
                pub fn clear_pending(&self) {
                    self.registers.ev_pending().modify(|r, w| unsafe { w.mask().bit(r.mask().bit()) });
                }
            }

            // trait: hal::delay::DelayUs
            impl $crate::hal::delay::DelayUs for $TIMERX {
                type Error = core::convert::Infallible;

                fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
                    let ticks: u32 = self.clk / 1_000_000 * us;

                    // start timer
                    self.registers.reload().write(|w| unsafe { w.value().bits(ticks) });
                    self.registers.enable().write(|w| w.enable().bit(true));

                    // wait for timer to hit zero
                    while self.registers.counter().read().value().bits() != 0 {}

                    // reset timer
                    self.registers.enable().write(|w| w.enable().bit(false));

                    Ok(())
                }
            }
        )+
    }
}
