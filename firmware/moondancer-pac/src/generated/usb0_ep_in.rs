#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: DATA,
    epno: EPNO,
    reset: RESET,
    stall: STALL,
    idle: IDLE,
    have: HAVE,
    pend: PEND,
    pid: PID,
    nak: NAK,
    _reserved9: [u8; 0x1c],
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - usb0_ep_in data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x04 - usb0_ep_in epno register"]
    #[inline(always)]
    pub const fn epno(&self) -> &EPNO {
        &self.epno
    }
    #[doc = "0x08 - usb0_ep_in reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x0c - usb0_ep_in stall register"]
    #[inline(always)]
    pub const fn stall(&self) -> &STALL {
        &self.stall
    }
    #[doc = "0x10 - usb0_ep_in idle register"]
    #[inline(always)]
    pub const fn idle(&self) -> &IDLE {
        &self.idle
    }
    #[doc = "0x14 - usb0_ep_in have register"]
    #[inline(always)]
    pub const fn have(&self) -> &HAVE {
        &self.have
    }
    #[doc = "0x18 - usb0_ep_in pend register"]
    #[inline(always)]
    pub const fn pend(&self) -> &PEND {
        &self.pend
    }
    #[doc = "0x1c - usb0_ep_in pid register"]
    #[inline(always)]
    pub const fn pid(&self) -> &PID {
        &self.pid
    }
    #[doc = "0x20 - usb0_ep_in nak register"]
    #[inline(always)]
    pub const fn nak(&self) -> &NAK {
        &self.nak
    }
    #[doc = "0x40 - usb0_ep_in ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x44 - usb0_ep_in ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x48 - usb0_ep_in ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "data (w) register accessor: usb0_ep_in data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "usb0_ep_in data register"]
pub mod data;
#[doc = "epno (rw) register accessor: usb0_ep_in epno register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epno::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epno::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epno`]
module"]
#[doc(alias = "epno")]
pub type EPNO = crate::Reg<epno::EPNO_SPEC>;
#[doc = "usb0_ep_in epno register"]
pub mod epno;
#[doc = "reset (w) register accessor: usb0_ep_in reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "usb0_ep_in reset register"]
pub mod reset;
#[doc = "stall (rw) register accessor: usb0_ep_in stall register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall`]
module"]
#[doc(alias = "stall")]
pub type STALL = crate::Reg<stall::STALL_SPEC>;
#[doc = "usb0_ep_in stall register"]
pub mod stall;
#[doc = "idle (r) register accessor: usb0_ep_in idle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle`]
module"]
#[doc(alias = "idle")]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "usb0_ep_in idle register"]
pub mod idle;
#[doc = "have (r) register accessor: usb0_ep_in have register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`have::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@have`]
module"]
#[doc(alias = "have")]
pub type HAVE = crate::Reg<have::HAVE_SPEC>;
#[doc = "usb0_ep_in have register"]
pub mod have;
#[doc = "pend (r) register accessor: usb0_ep_in pend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pend`]
module"]
#[doc(alias = "pend")]
pub type PEND = crate::Reg<pend::PEND_SPEC>;
#[doc = "usb0_ep_in pend register"]
pub mod pend;
#[doc = "pid (rw) register accessor: usb0_ep_in pid register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "pid")]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "usb0_ep_in pid register"]
pub mod pid;
#[doc = "nak (r) register accessor: usb0_ep_in nak register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nak::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nak`]
module"]
#[doc(alias = "nak")]
pub type NAK = crate::Reg<nak::NAK_SPEC>;
#[doc = "usb0_ep_in nak register"]
pub mod nak;
#[doc = "ev_status (r) register accessor: usb0_ep_in ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
#[doc(alias = "ev_status")]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb0_ep_in ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: usb0_ep_in ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb0_ep_in ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: usb0_ep_in ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb0_ep_in ev_enable register"]
pub mod ev_enable;
