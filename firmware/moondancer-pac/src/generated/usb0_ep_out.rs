#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    data: DATA,
    data_ep: DATA_EP,
    reset: RESET,
    epno: EPNO,
    enable: ENABLE,
    prime: PRIME,
    stall: STALL,
    have: HAVE,
    pend: PEND,
    address: ADDRESS,
    pid: PID,
    _reserved11: [u8; 0x14],
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - usb0_ep_out data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x04 - usb0_ep_out data_ep register"]
    #[inline(always)]
    pub const fn data_ep(&self) -> &DATA_EP {
        &self.data_ep
    }
    #[doc = "0x08 - usb0_ep_out reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x0c - usb0_ep_out epno register"]
    #[inline(always)]
    pub const fn epno(&self) -> &EPNO {
        &self.epno
    }
    #[doc = "0x10 - usb0_ep_out enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x14 - usb0_ep_out prime register"]
    #[inline(always)]
    pub const fn prime(&self) -> &PRIME {
        &self.prime
    }
    #[doc = "0x18 - usb0_ep_out stall register"]
    #[inline(always)]
    pub const fn stall(&self) -> &STALL {
        &self.stall
    }
    #[doc = "0x1c - usb0_ep_out have register"]
    #[inline(always)]
    pub const fn have(&self) -> &HAVE {
        &self.have
    }
    #[doc = "0x20 - usb0_ep_out pend register"]
    #[inline(always)]
    pub const fn pend(&self) -> &PEND {
        &self.pend
    }
    #[doc = "0x24 - usb0_ep_out address register"]
    #[inline(always)]
    pub const fn address(&self) -> &ADDRESS {
        &self.address
    }
    #[doc = "0x28 - usb0_ep_out pid register"]
    #[inline(always)]
    pub const fn pid(&self) -> &PID {
        &self.pid
    }
    #[doc = "0x40 - usb0_ep_out ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x44 - usb0_ep_out ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x48 - usb0_ep_out ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "data (r) register accessor: usb0_ep_out data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "usb0_ep_out data register"]
pub mod data;
#[doc = "data_ep (r) register accessor: usb0_ep_out data_ep register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_ep::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_ep`]
module"]
pub type DATA_EP = crate::Reg<data_ep::DATA_EP_SPEC>;
#[doc = "usb0_ep_out data_ep register"]
pub mod data_ep;
#[doc = "reset (w) register accessor: usb0_ep_out reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "usb0_ep_out reset register"]
pub mod reset;
#[doc = "epno (rw) register accessor: usb0_ep_out epno register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epno::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epno::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epno`]
module"]
pub type EPNO = crate::Reg<epno::EPNO_SPEC>;
#[doc = "usb0_ep_out epno register"]
pub mod epno;
#[doc = "enable (rw) register accessor: usb0_ep_out enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "usb0_ep_out enable register"]
pub mod enable;
#[doc = "prime (w) register accessor: usb0_ep_out prime register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prime::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prime`]
module"]
pub type PRIME = crate::Reg<prime::PRIME_SPEC>;
#[doc = "usb0_ep_out prime register"]
pub mod prime;
#[doc = "stall (rw) register accessor: usb0_ep_out stall register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall`]
module"]
pub type STALL = crate::Reg<stall::STALL_SPEC>;
#[doc = "usb0_ep_out stall register"]
pub mod stall;
#[doc = "have (r) register accessor: usb0_ep_out have register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`have::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@have`]
module"]
pub type HAVE = crate::Reg<have::HAVE_SPEC>;
#[doc = "usb0_ep_out have register"]
pub mod have;
#[doc = "pend (r) register accessor: usb0_ep_out pend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pend`]
module"]
pub type PEND = crate::Reg<pend::PEND_SPEC>;
#[doc = "usb0_ep_out pend register"]
pub mod pend;
#[doc = "address (rw) register accessor: usb0_ep_out address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "usb0_ep_out address register"]
pub mod address;
#[doc = "pid (rw) register accessor: usb0_ep_out pid register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "usb0_ep_out pid register"]
pub mod pid;
#[doc = "ev_status (r) register accessor: usb0_ep_out ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb0_ep_out ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: usb0_ep_out ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb0_ep_out ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: usb0_ep_out ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb0_ep_out ev_enable register"]
pub mod ev_enable;
