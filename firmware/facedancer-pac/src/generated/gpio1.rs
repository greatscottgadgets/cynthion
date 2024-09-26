#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: MODE,
    input: INPUT,
    output: OUTPUT,
    set_clr: SET_CLR,
}
impl RegisterBlock {
    #[doc = "0x00 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x02 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn input(&self) -> &INPUT {
        &self.input
    }
    #[doc = "0x03 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn output(&self) -> &OUTPUT {
        &self.output
    }
    #[doc = "0x04 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn set_clr(&self) -> &SET_CLR {
        &self.set_clr
    }
}
#[doc = "Mode (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "Mode")]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod mode;
#[doc = "Input (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
#[doc(alias = "Input")]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod input;
#[doc = "Output (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "Output")]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod output;
#[doc = "SetClr (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_clr`]
module"]
#[doc(alias = "SetClr")]
pub type SET_CLR = crate::Reg<set_clr::SET_CLR_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod set_clr;
