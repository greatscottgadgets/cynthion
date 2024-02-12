// - Error --------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FirmwareError {
    Unknown,
}

// trait:: core::fmt::Display
impl core::fmt::Display for FirmwareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

// trait: core::error::Error
#[cfg(feature = "nightly")]
impl core::error::Error for FirmwareError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        use FirmwareError::*;
        match self {
            Unknown => "TODO Unknown",
        }
    }
}
