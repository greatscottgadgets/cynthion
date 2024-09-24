#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    connect: CONNECT,
    speed: SPEED,
    low_speed_only: LOW_SPEED_ONLY,
    full_speed_only: FULL_SPEED_ONLY,
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - usb1 connect register"]
    #[inline(always)]
    pub const fn connect(&self) -> &CONNECT {
        &self.connect
    }
    #[doc = "0x04 - usb1 speed register"]
    #[inline(always)]
    pub const fn speed(&self) -> &SPEED {
        &self.speed
    }
    #[doc = "0x08 - usb1 low_speed_only register"]
    #[inline(always)]
    pub const fn low_speed_only(&self) -> &LOW_SPEED_ONLY {
        &self.low_speed_only
    }
    #[doc = "0x0c - usb1 full_speed_only register"]
    #[inline(always)]
    pub const fn full_speed_only(&self) -> &FULL_SPEED_ONLY {
        &self.full_speed_only
    }
    #[doc = "0x10 - usb1 ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x14 - usb1 ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x18 - usb1 ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "connect (rw) register accessor: usb1 connect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`connect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`connect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@connect`]
module"]
#[doc(alias = "connect")]
pub type CONNECT = crate::Reg<connect::CONNECT_SPEC>;
#[doc = "usb1 connect register"]
pub mod connect;
#[doc = "speed (r) register accessor: usb1 speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`speed::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@speed`]
module"]
#[doc(alias = "speed")]
pub type SPEED = crate::Reg<speed::SPEED_SPEC>;
#[doc = "usb1 speed register"]
pub mod speed;
#[doc = "low_speed_only (rw) register accessor: usb1 low_speed_only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_speed_only::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_speed_only::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_speed_only`]
module"]
#[doc(alias = "low_speed_only")]
pub type LOW_SPEED_ONLY = crate::Reg<low_speed_only::LOW_SPEED_ONLY_SPEC>;
#[doc = "usb1 low_speed_only register"]
pub mod low_speed_only;
#[doc = "full_speed_only (rw) register accessor: usb1 full_speed_only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`full_speed_only::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`full_speed_only::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@full_speed_only`]
module"]
#[doc(alias = "full_speed_only")]
pub type FULL_SPEED_ONLY = crate::Reg<full_speed_only::FULL_SPEED_ONLY_SPEC>;
#[doc = "usb1 full_speed_only register"]
pub mod full_speed_only;
#[doc = "ev_status (r) register accessor: usb1 ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
#[doc(alias = "ev_status")]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb1 ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: usb1 ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb1 ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: usb1 ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb1 ev_enable register"]
pub mod ev_enable;
