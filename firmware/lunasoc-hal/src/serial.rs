/// Re-export hal serial error type
pub use crate::hal::serial::ErrorKind as Error;

#[macro_export]
macro_rules! impl_serial {
    ($(
        $SERIALX:ident: $PACUARTX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $SERIALX {
                registers: $PACUARTX,
            }

            // lifecycle
            impl $SERIALX {
                /// Create a new `Serial` from the [`UART`](crate::pac::UART) peripheral.
                pub fn new(registers: $PACUARTX) -> Self {
                    Self { registers }
                }

                /// Release the [`Uart`](crate::pac::UART) peripheral and consume self.
                pub fn free(self) -> $PACUARTX {
                    self.registers
                }

                /// Obtain a static `Serial` instance for use in e.g. interrupt handlers
                ///
                /// # Safety
                ///
                /// 'Tis thine responsibility, that which thou doth summon.
                pub unsafe fn summon() -> Self {
                    Self {
                        registers: <$PACUARTX>::steal(),
                    }
                }
            }

            // trait: From
            impl From<$PACUARTX> for $SERIALX {
                fn from(registers: $PACUARTX) -> $SERIALX {
                    $SERIALX::new(registers)
                }
            }

            // trait: core::fmt::Write
            impl core::fmt::Write for $SERIALX {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    use $crate::hal::serial::Write;
                    self.write(s.as_bytes()).ok();
                    Ok(())
                }
            }

            // - embedded_hal 1.0 traits --------------------------------------

            // trait: hal::serial::ErrorType
            impl $crate::hal::serial::ErrorType for $SERIALX {
                type Error = $crate::serial::Error;
            }

            // trait: hal::serial::Write
            impl $crate::hal::serial::Write<u8> for $SERIALX {
                fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
                    for &byte in buffer {
                        $crate::nb::block!(
                            <$SERIALX as $crate::hal_nb::serial::Write<u8>>::write(self, byte)
                        )?;
                    }
                    Ok(())
                }

                fn flush(&mut self) -> Result<(), Self::Error> {
                    $crate::nb::block!(
                        <$SERIALX as $crate::hal_nb::serial::Write<u8>>::flush(self)
                    )
                }
            }

            // trait: hal_nb::serial::Write
            impl $crate::hal_nb::serial::Write<u8> for $SERIALX {
                fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.tx_rdy().read().tx_rdy().bit() {
                        self.registers.tx_data().write(|w| unsafe { w.tx_data().bits(byte.into()) });
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }

                fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.tx_rdy().read().tx_rdy().bit() {
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }
            }


            // - embedded_hal 0.x traits --------------------------------------

            // trait: hal::serial::Write
            impl $crate::hal_0::serial::Write<u8> for $SERIALX {
                type Error = $crate::serial::Error;

                fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.tx_rdy().read().tx_rdy().bit() {
                        self.registers.tx_data().write(|w| unsafe { w.tx_data().bits(byte.into()) });
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }
                fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.tx_rdy().read().tx_rdy().bit() {
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }
            }

            // trait: hal::blocking::serial::write::Default
            impl $crate::hal_0::blocking::serial::write::Default<u8> for $SERIALX {}

        )+
    }
}
