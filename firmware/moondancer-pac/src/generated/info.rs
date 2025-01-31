#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Version register major : Contains the Cynthion hardware major revision number. minor : Contains the Cynthion hardware minor revision number."]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "version (rw) register accessor: Version register major : Contains the Cynthion hardware major revision number. minor : Contains the Cynthion hardware minor revision number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "version")]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register major : Contains the Cynthion hardware major revision number. minor : Contains the Cynthion hardware minor revision number."]
pub mod version;
