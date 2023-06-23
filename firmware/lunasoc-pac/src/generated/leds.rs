#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - leds output register"]
    pub output: OUTPUT,
}
#[doc = "output (w) register accessor: an alias for `Reg<OUTPUT_SPEC>`"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "leds output register"]
pub mod output;
