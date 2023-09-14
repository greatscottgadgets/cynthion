#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - usb0 connect register"]
    pub connect: CONNECT,
    #[doc = "0x04 - usb0 speed register"]
    pub speed: SPEED,
    #[doc = "0x08 - usb0 low_speed_only register"]
    pub low_speed_only: LOW_SPEED_ONLY,
    #[doc = "0x0c - usb0 full_speed_only register"]
    pub full_speed_only: FULL_SPEED_ONLY,
    #[doc = "0x10 - usb0 ev_status register"]
    pub ev_status: EV_STATUS,
    #[doc = "0x14 - usb0 ev_pending register"]
    pub ev_pending: EV_PENDING,
    #[doc = "0x18 - usb0 ev_enable register"]
    pub ev_enable: EV_ENABLE,
}
#[doc = "connect (rw) register accessor: an alias for `Reg<CONNECT_SPEC>`"]
pub type CONNECT = crate::Reg<connect::CONNECT_SPEC>;
#[doc = "usb0 connect register"]
pub mod connect;
#[doc = "speed (r) register accessor: an alias for `Reg<SPEED_SPEC>`"]
pub type SPEED = crate::Reg<speed::SPEED_SPEC>;
#[doc = "usb0 speed register"]
pub mod speed;
#[doc = "low_speed_only (rw) register accessor: an alias for `Reg<LOW_SPEED_ONLY_SPEC>`"]
pub type LOW_SPEED_ONLY = crate::Reg<low_speed_only::LOW_SPEED_ONLY_SPEC>;
#[doc = "usb0 low_speed_only register"]
pub mod low_speed_only;
#[doc = "full_speed_only (rw) register accessor: an alias for `Reg<FULL_SPEED_ONLY_SPEC>`"]
pub type FULL_SPEED_ONLY = crate::Reg<full_speed_only::FULL_SPEED_ONLY_SPEC>;
#[doc = "usb0 full_speed_only register"]
pub mod full_speed_only;
#[doc = "ev_status (r) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb0 ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb0 ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb0 ev_enable register"]
pub mod ev_enable;
