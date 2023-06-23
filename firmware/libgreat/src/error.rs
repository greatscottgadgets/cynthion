#![allow(unused_variables)] // TODO

/**
 * Rust error-handling continues to be somewhat of a chore in no_std.
 *
 * Some light reading:
 *
 *   * https://doc.rust-lang.org/rust-by-example/error/multiple_error_types.html
 *   * https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
 *   * https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
 *   * https://www.sheshbabu.com/posts/rust-error-handling/
 *   * https://richard.dallaway.com/posts/2020-01-20-rust-error-chaining/
 *
 * Useful documentation:
 *
 *   * https://doc.rust-lang.org/beta/core/error/trait.Error.html
 *
 */

// /// The libgreat Error trait
// pub trait GreatErrorTrait: core::fmt::Debug {
//     type Error: GreatErrorTrait;
//     fn kind(&self) -> Self::Error;
// }

// /// Defines an error type, to be used by any other traits.
// pub trait GreatErrorType {
//     /// Error type
//     type Error: GreatErrorTrait;
// }

// impl<T: GreatErrorType> GreatErrorType for &mut T {
//     type Error = T::Error;
// }

/// Result<T>
///
/// TODO consider switching to a single global enum
//pub type Result<T> = core::result::Result<T, &'static (dyn core::error::Error + 'static)>;
pub type GreatResult<T> = core::result::Result<T, GreatError>;

/// GreatError
#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum GreatError {
    // from libgreat/errno.h
    NotOwner = 1,                 // EPERM    - Not owner
    NoSuchFileOrDirectory = 2,    // ENOENT   - No such file or directory
    InterruptedSystemCall = 4,    // EINTR    - Interrupted system call
    ArgumentListTooLong = 7,      // E2BIG    - Arg list too long
    BadAddress = 14,              // EFAULT   - Bad address
    DeviceOrResourceBusy = 15,    // EBUSY    - Device or resource busy
    InvalidArgument = 22,         // EINVAL   - Invalid argument
    NoSpaceLeftOnDevice = 28,     // ENOSPC   - No space left on device
    NoMessageOfType = 35,         // ENOMSG   - No message of desired type
    NoData = 61,                  // ENODATA  - No data
    BadMessage = 77,              // EBADMSG  - Bad message
    NoBufferSpaceAvailable = 105, // ENOBUFS - No buffer space available

    // TODO consider just using libgreat/errno.h errors above
    GcpClassNotFound(crate::gcp::class::ClassId) = 0xf000,
    GcpVerbNotFound(crate::gcp::class::ClassId, u32) = 0xf001,
    GcpUnknownVerbDescriptor(u8) = 0xf002,

    // handy
    Message(&'static str) = 0xffff,

    #[cfg(feature = "nightly")]
    DynError(&'static (dyn core::error::Error)),
}

// impl<'a> From<&'a GreatError> for &'a dyn core::error::Error {
//     fn from(error: &'a GreatError) -> Self {
//         error
//     }
// }

impl core::fmt::Display for GreatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

// impl core::error::Error for GreatError {
//     #[allow(deprecated)]
//     fn description(&self) -> &str {
//         use GreatError::*;
//         match self {
//             Message(message) => message,
//             // TODO - move these to gcp errors
//             GcpInvalidArguments => "gcp invalid arguments",
//             GcpClassNotFound => "gcp class not found",
//             GcpVerbNotFound => "gcp verb not found",
//             GcpUnknownVerbDescriptor => "gcp unknown verb descriptor",
//         }
//     }
// }

// impl GreatErrorTrait for GreatError {
//     type Error = Self;
//     fn kind(&self) -> Self::Error {
//         *self
//     }
// }

#[cfg(test)]
mod tests {
    //use super::*;

    // - fixtures -------------------------------------------------------------

    /// A Custom ErrorKind
    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[non_exhaustive] // ... is a double-edged sword
    pub enum CustomErrorKind {
        One,
        Two,
        Unknown,
    }

    #[cfg(not(feature = "nightly"))]
    // trait: core::fmt::Display
    impl core::fmt::Display for CustomErrorKind {
        #[allow(deprecated)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(&self, f)
        }
    }

    #[cfg(feature = "nightly")]
    // trait: core::fmt::Display
    impl core::fmt::Display for CustomErrorKind {
        #[allow(deprecated)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use core::error::Error;
            write!(f, "{}", self.description())
        }
    }

    // trait: libgreat::Error
    // impl GreatErrorTrait for CustomErrorKind {
    //     type Error = CustomErrorKind;
    //     fn kind(&self) -> Self::Error {
    //         *self
    //     }
    // }

    // trait: core::num::TryFromIntError
    impl core::convert::From<core::num::TryFromIntError> for CustomErrorKind {
        fn from(_error: core::num::TryFromIntError) -> Self {
            CustomErrorKind::One
        }
    }

    #[cfg(feature = "nightly")]
    // trait: core::error::Error
    impl core::error::Error for CustomErrorKind {
        #[allow(deprecated)]
        fn description(&self) -> &str {
            use CustomErrorKind::*;
            match self {
                One => "This is a One error",
                Two => "This is a Two error",
                Unknown => "This is an Unknown error",
            }
        }
    }

    /// returns a `CustomErrorKind`
    fn result_custom(n: u32) -> core::result::Result<u32, CustomErrorKind> {
        if n % 32 == 0 {
            Ok(n)
        } else {
            Err(CustomErrorKind::Two)
        }
    }

    #[cfg(feature = "nightly")]
    /// returns a `&'static (dyn core::error::Error + 'static)`
    fn result_core_error(n: u32) -> core::result::Result<u32, &'static (dyn core::error::Error)> {
        if n % 32 == 0 {
            Ok(n)
        } else {
            Err(&CustomErrorKind::Unknown)
        }
    }

    // - tests ----------------------------------------------------------------

    #[test]
    fn test_error() {
        match result_custom(31) {
            Ok(_n) => (),
            Err(CustomErrorKind::Unknown) => {
                println!("Unknown Error");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn test_nightly() {
        match result_core_error(31) {
            Ok(_n) => (),
            Err(e) => {
                println!("Error: {}", e);
                if e.is::<CustomErrorKind>() {
                    println!("  ... which is a custom error");
                }
            }
        }
    }

    // #[test]
    // fn test_great_error_trait() {
    //     match result_custom(31) {
    //         Ok(_n) => (),
    //         Err(error) => match error.kind() {
    //             CustomErrorKind::Unknown => {
    //                 println!("Unknown Error");
    //             }
    //             _ => {
    //                 println!("Error: {}", error);
    //             }
    //         },
    //     }
    // }
}
