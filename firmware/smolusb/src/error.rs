/// [`smolusb`](crate) Error type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SmolError {
    FailedConversion,
}

// trait:: core::fmt::Display
impl core::fmt::Display for SmolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

// trait: core::convert::From<core::num::TryFromIntError>
impl core::convert::From<core::num::TryFromIntError> for SmolError {
    fn from(_error: core::num::TryFromIntError) -> Self {
        SmolError::FailedConversion
    }
}

// trait: libgreat::error::Error
// impl libgreat::error::GreatErrorTrait for SmolError {
//     type Error = Self;
//     fn kind(&self) -> Self::Error {
//         *self
//     }
// }

#[cfg(feature = "nightly")]
// trait: core::error::Error
impl core::error::Error for SmolError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        use SmolError::*;
        match self {
            FailedConversion => "Failed to convert packet value",
        }
    }
}

/// [`smolusb`](crate) [`Result`] type.
pub type SmolResult<T> = core::result::Result<T, SmolError>;
