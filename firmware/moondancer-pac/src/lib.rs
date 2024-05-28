//! Peripheral access API for Luna System-on-Chip designs generated using svd2rust.

#![no_std]
#![allow(clippy::inline_always)]

#[cfg(all(feature = "minerva", feature = "vexriscv"))]
compile_error!(r#"Only one of the "minerva" or "vexriscv" features can be selected"#);

#[macro_use]
mod macros;

pub mod cpu;
pub mod csr;
pub mod register {
    #[cfg(feature = "minerva")]
    pub use crate::cpu::minerva::register::*;
    #[cfg(feature = "vexriscv")]
    pub use crate::cpu::vexriscv::register::*;
}

pub mod clock {
    const SYSTEM_CLOCK_FREQUENCY: u32 = 60_000_000;

    #[must_use]
    pub const fn sysclk() -> u32 {
        SYSTEM_CLOCK_FREQUENCY
    }
}

#[deny(dead_code)]
#[deny(improper_ctypes)]
#[deny(missing_docs)]
#[deny(no_mangle_generic_items)]
#[deny(non_shorthand_field_patterns)]
#[deny(overflowing_literals)]
#[deny(path_statements)]
#[deny(patterns_in_fns_without_body)]
#[deny(unconditional_recursion)]
#[deny(unused_allocation)]
#[deny(unused_comparisons)]
#[deny(unused_parens)]
#[deny(while_true)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::semicolon_if_nothing_returned)]
mod generated;

pub use generated::generic::*;
pub use generated::*;
