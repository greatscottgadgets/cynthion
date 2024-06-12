#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    moder: MODER,
    odr: ODR,
    idr: IDR,
    _reserved3: [u8; 0x04],
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - gpiob moder register"]
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    #[doc = "0x04 - gpiob odr register"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    #[doc = "0x08 - gpiob idr register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x10 - gpiob ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x14 - gpiob ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x18 - gpiob ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "moder (rw) register accessor: gpiob moder register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`]
module"]
#[doc(alias = "moder")]
pub type MODER = crate::Reg<moder::MODER_SPEC>;
#[doc = "gpiob moder register"]
pub mod moder;
#[doc = "odr (w) register accessor: gpiob odr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
#[doc(alias = "odr")]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "gpiob odr register"]
pub mod odr;
#[doc = "idr (r) register accessor: gpiob idr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "idr")]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "gpiob idr register"]
pub mod idr;
#[doc = "ev_status (r) register accessor: gpiob ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
#[doc(alias = "ev_status")]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "gpiob ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: gpiob ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "gpiob ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: gpiob ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "gpiob ev_enable register"]
pub mod ev_enable;
