//! A simple logger for the `log` crate which can log to any object
//! implementing `Write`

use crate::{hal, pac};

use log::{Level, LevelFilter, Metadata, Record};

use core::cell::RefCell;
use core::fmt::Write;

// - initialization -----------------------------------------------------------

use core::cell::Cell;
use critical_section::Mutex;
static LOGGER: Mutex<Cell<WriteLogger<hal::Serial>>> = Mutex::new(Cell::new(WriteLogger::new()));

pub fn init(writer: hal::Serial) {
    critical_section::with(|cs| {
        LOGGER.borrow(cs).set(
            WriteLogger {
                writer: Some(writer),
                level: Level::Trace,
            }
        );
    });

    // TODO we need support for atomics to use log::set_logger()
    unsafe { log::set_logger_racy(&LOGGER) }
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();
}

// - implementation -----------------------------------------------------------

/// WriteLogger
pub struct WriteLogger<W>
where
    W: Write + Send,
{
    pub writer: Option<W>,
    pub level: Level,
}

impl <W> WriteLogger<W>
where
    W: Write + Send, {
    fn new() -> Self {
        Self {
            writer: None,
            level: Level::Trace,
        }
    }
}

impl<W> log::Log for WriteLogger<W>
where
    W: Write + Send,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        /*riscv::interrupt::free(|| match self.writer.as_mut() {
            Some(writer) => {
                writeln!(writer, "{} - {}", record.level(), record.args())
                    .expect("Logger failed to write to device");
            }
            None => {
                panic!("Logger has not been initialized");
            }
        })*/
    }

    fn flush(&self) {}
}

// TODO add support for critical-section crate
// TODO implement a riscv::interrupt::Mutex
unsafe impl<W: Write + Send> Sync for WriteLogger<W> {}
