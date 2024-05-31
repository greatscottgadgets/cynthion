#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: DATA,
    reset: RESET,
    epno: EPNO,
    have: HAVE,
    pend: PEND,
    address: ADDRESS,
    _reserved6: [u8; 0x08],
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - usb1_ep_control data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x04 - usb1_ep_control reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x08 - usb1_ep_control epno register"]
    #[inline(always)]
    pub const fn epno(&self) -> &EPNO {
        &self.epno
    }
    #[doc = "0x0c - usb1_ep_control have register"]
    #[inline(always)]
    pub const fn have(&self) -> &HAVE {
        &self.have
    }
    #[doc = "0x10 - usb1_ep_control pend register"]
    #[inline(always)]
    pub const fn pend(&self) -> &PEND {
        &self.pend
    }
    #[doc = "0x14 - usb1_ep_control address register"]
    #[inline(always)]
    pub const fn address(&self) -> &ADDRESS {
        &self.address
    }
    #[doc = "0x20 - usb1_ep_control ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x24 - usb1_ep_control ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x28 - usb1_ep_control ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "data (r) register accessor: usb1_ep_control data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "usb1_ep_control data register"]
pub mod data;
#[doc = "reset (w) register accessor: usb1_ep_control reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "usb1_ep_control reset register"]
pub mod reset;
#[doc = "epno (r) register accessor: usb1_ep_control epno register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epno::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epno`]
module"]
#[doc(alias = "epno")]
pub type EPNO = crate::Reg<epno::EPNO_SPEC>;
#[doc = "usb1_ep_control epno register"]
pub mod epno;
#[doc = "have (r) register accessor: usb1_ep_control have register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`have::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@have`]
module"]
#[doc(alias = "have")]
pub type HAVE = crate::Reg<have::HAVE_SPEC>;
#[doc = "usb1_ep_control have register"]
pub mod have;
#[doc = "pend (r) register accessor: usb1_ep_control pend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pend`]
module"]
#[doc(alias = "pend")]
pub type PEND = crate::Reg<pend::PEND_SPEC>;
#[doc = "usb1_ep_control pend register"]
pub mod pend;
#[doc = "address (rw) register accessor: usb1_ep_control address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "address")]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "usb1_ep_control address register"]
pub mod address;
#[doc = "ev_status (r) register accessor: usb1_ep_control ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
#[doc(alias = "ev_status")]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb1_ep_control ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: usb1_ep_control ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb1_ep_control ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: usb1_ep_control ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb1_ep_control ev_enable register"]
pub mod ev_enable;
