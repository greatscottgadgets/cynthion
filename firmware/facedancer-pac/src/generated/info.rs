#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version_major: VERSION_MAJOR,
    version_minor: VERSION_MINOR,
}
impl RegisterBlock {
    #[doc = "0x00 - info version_major register"]
    #[inline(always)]
    pub const fn version_major(&self) -> &VERSION_MAJOR {
        &self.version_major
    }
    #[doc = "0x04 - info version_minor register"]
    #[inline(always)]
    pub const fn version_minor(&self) -> &VERSION_MINOR {
        &self.version_minor
    }
}
#[doc = "version_major (r) register accessor: info version_major register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_major::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_major`]
module"]
#[doc(alias = "version_major")]
pub type VERSION_MAJOR = crate::Reg<version_major::VERSION_MAJOR_SPEC>;
#[doc = "info version_major register"]
pub mod version_major;
#[doc = "version_minor (r) register accessor: info version_minor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_minor::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_minor`]
module"]
#[doc(alias = "version_minor")]
pub type VERSION_MINOR = crate::Reg<version_minor::VERSION_MINOR_SPEC>;
#[doc = "info version_minor register"]
pub mod version_minor;
