#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    enable: ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - advertiser enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
}
#[doc = "enable (rw) register accessor: advertiser enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "advertiser enable register"]
pub mod enable;
