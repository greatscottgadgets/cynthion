/// USB Error type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    Timeout,
}

// trait:: core::fmt::Display
impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

// trait: libgreat::error::Error
// impl libgreat::error::GreatErrorTrait for ErrorKind {
//     type Error = Self;
//     fn kind(&self) -> Self::Error {
//         *self
//     }
// }

#[cfg(feature = "nightly")]
// trait: core::error::Error
impl core::error::Error for ErrorKind {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        use ErrorKind::*;
        match self {
            Timeout => "Blocking operation timed-out",
        }
    }
}
