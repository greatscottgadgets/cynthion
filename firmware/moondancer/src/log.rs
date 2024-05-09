//! A simple logger for Cynthion's serial ports.

use core::fmt::Write as _;
use core::ptr::addr_of_mut;

use log::{Level, LevelFilter, Metadata, Record};

use hal::hal::serial::Write as _;

use crate::hal;

// - initialization -----------------------------------------------------------

static mut LOGGER: CynthionLogger = CynthionLogger::new(Port::Both, Level::Trace);

/// Initializes logging using the given serial port
///
/// # Panics
///
/// This function will panic if the logger cannot be initialized.
pub fn init() {
    let logger = unsafe { &mut *addr_of_mut!(LOGGER) };

    #[cfg(target_has_atomic)]
    {
        match log::set_logger(logger).map(|()| log::set_max_level(LevelFilter::Trace)) {
            Ok(()) => (),
            Err(_e) => {
                panic!("Failed to set logger");
            }
        }
    }

    #[cfg(not(target_has_atomic))]
    {
        match unsafe { log::set_logger_racy(logger) }
            .map(|()| log::set_max_level(LevelFilter::Trace))
        {
            Ok(()) => (),
            Err(_e) => {
                panic!("Failed to set logger");
            }
        }
    }
}

/// Override the default Uart (Uart0) to use for the logger
pub fn set_port(port: Port) {
    let logger = unsafe { &mut *addr_of_mut!(LOGGER) };
    logger.set_port(port);
}

// - implementation -----------------------------------------------------------

pub enum Port {
    Uart0,
    Uart1,
    Both,
}

/// Logger for objects implementing [`Write`] and [`Send`].
pub struct CynthionLogger {
    pub port: Port,
    pub level: Level,
}

impl CynthionLogger {
    #[must_use]
    pub const fn new(port: Port, level: Level) -> Self {
        Self { port, level }
    }

    pub fn set_port(&mut self, port: Port) {
        self.port = port;
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}

impl log::Log for CynthionLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    /// Write the given record to the log
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match self.port {
            Port::Uart0 => {
                let mut writer = unsafe { hal::Serial0::summon() };
                writeln!(writer, "{}\t{}", record.level(), record.args()).unwrap_or(());
            }
            Port::Uart1 => {
                let mut writer = unsafe { hal::Serial1::summon() };
                writeln!(writer, "{}\t{}", record.level(), record.args()).unwrap_or(());
            }
            Port::Both => {
                let mut writer = unsafe { hal::Serial0::summon() };
                writeln!(writer, "{}\t{}", record.level(), record.args()).unwrap_or(());
                let mut writer = unsafe { hal::Serial1::summon() };
                writeln!(writer, "{}\t{}", record.level(), record.args()).unwrap_or(());
            }
        }
    }

    fn flush(&self) {
        match self.port {
            Port::Uart0 => {
                let mut writer = unsafe { hal::Serial0::summon() };
                writer.flush().ok();
            }
            Port::Uart1 => {
                let mut writer = unsafe { hal::Serial1::summon() };
                writer.flush().ok();
            }
            Port::Both => {
                let mut writer = unsafe { hal::Serial0::summon() };
                writer.flush().ok();
                let mut writer = unsafe { hal::Serial1::summon() };
                writer.flush().ok();
            }
        }
    }
}

// - format! ------------------------------------------------------------------

/// format! macro for `no_std`, `no_alloc` environments
///
/// Props: <https://stackoverflow.com/questions/50200268/>
/// Props: <https://github.com/Simsys/arrform>
///
/// TODO Re-use buffer

#[cfg(not(feature = "alloc"))]
pub mod format_nostd {
    pub const SIZE: usize = 128;

    #[macro_export]
    macro_rules! _format {
        ($($arg:tt)*) => {
            {
                use core::fmt::Write;
                use moondancer::log::format_nostd::BufferWriter;
                use moondancer::log::format_nostd::SIZE;
                let mut buffer = [0u8; SIZE];
                let mut writer = BufferWriter::new(buffer);
                write!(&mut writer, $($arg)*).unwrap();
                writer
            }
        };
    }
    pub use _format as format;

    pub struct BufferWriter {
        buffer: [u8; SIZE],
        cursor: usize,
    }

    impl BufferWriter {
        #[must_use]
        pub fn new(buffer: [u8; SIZE]) -> Self {
            BufferWriter { buffer, cursor: 0 }
        }

        pub fn reset(&mut self) {
            self.cursor = 0;
        }

        #[must_use]
        pub fn as_bytes(&self) -> &[u8] {
            &self.buffer[0..self.cursor]
        }

        #[must_use]
        pub fn as_str(&self) -> &str {
            core::str::from_utf8(&self.buffer[0..self.cursor]).unwrap_or("invalid utf-8 string")
        }
    }

    impl core::fmt::Write for BufferWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let len = self.buffer.len();
            for (i, &b) in self.buffer[self.cursor..len]
                .iter_mut()
                .zip(s.as_bytes().iter())
            {
                *i = b;
            }
            self.cursor = usize::min(len, self.cursor + s.as_bytes().len());
            Ok(())
        }
    }
}

pub use format_nostd::format;
