#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    output: OUTPUT,
}
impl RegisterBlock {
    #[doc = "0x00 - leds output register"]
    #[inline(always)]
    pub const fn output(&self) -> &OUTPUT {
        &self.output
    }
}
#[doc = "output (w) register accessor: leds output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "output")]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "leds output register"]
pub mod output;
