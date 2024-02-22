//! smolusb Error type

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    Timeout(usize),
    Overflow(usize),
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

#[cfg(feature = "nightly")]
impl core::error::Error for ErrorKind {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        use ErrorKind::*;
        match self {
            Timeout(_) => "Blocking operation timed-out",
            Overflow(_) => "Read operation overflowed receive buffer",
        }
    }
}
