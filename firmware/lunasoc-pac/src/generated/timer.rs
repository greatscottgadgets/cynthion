#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - timer reload register"]
    pub reload: RELOAD,
    #[doc = "0x04 - timer en register"]
    pub en: EN,
    #[doc = "0x08 - timer ctr register"]
    pub ctr: CTR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - timer ev_status register"]
    pub ev_status: EV_STATUS,
    #[doc = "0x14 - timer ev_pending register"]
    pub ev_pending: EV_PENDING,
    #[doc = "0x18 - timer ev_enable register"]
    pub ev_enable: EV_ENABLE,
}
#[doc = "reload (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "timer reload register"]
pub mod reload;
#[doc = "en (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "timer en register"]
pub mod en;
#[doc = "ctr (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "timer ctr register"]
pub mod ctr;
#[doc = "ev_status (r) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "timer ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "timer ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "timer ev_enable register"]
pub mod ev_enable;
