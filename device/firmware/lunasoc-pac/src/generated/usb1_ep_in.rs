#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - usb1_ep_in data register"]
    pub data: DATA,
    #[doc = "0x04 - usb1_ep_in epno register"]
    pub epno: EPNO,
    #[doc = "0x08 - usb1_ep_in reset register"]
    pub reset: RESET,
    #[doc = "0x0c - usb1_ep_in stall register"]
    pub stall: STALL,
    #[doc = "0x10 - usb1_ep_in idle register"]
    pub idle: IDLE,
    #[doc = "0x14 - usb1_ep_in have register"]
    pub have: HAVE,
    #[doc = "0x18 - usb1_ep_in pend register"]
    pub pend: PEND,
    #[doc = "0x1c - usb1_ep_in pid register"]
    pub pid: PID,
    #[doc = "0x20 - usb1_ep_in ev_status register"]
    pub ev_status: EV_STATUS,
    #[doc = "0x24 - usb1_ep_in ev_pending register"]
    pub ev_pending: EV_PENDING,
    #[doc = "0x28 - usb1_ep_in ev_enable register"]
    pub ev_enable: EV_ENABLE,
}
#[doc = "data (w) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "usb1_ep_in data register"]
pub mod data;
#[doc = "epno (rw) register accessor: an alias for `Reg<EPNO_SPEC>`"]
pub type EPNO = crate::Reg<epno::EPNO_SPEC>;
#[doc = "usb1_ep_in epno register"]
pub mod epno;
#[doc = "reset (w) register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "usb1_ep_in reset register"]
pub mod reset;
#[doc = "stall (rw) register accessor: an alias for `Reg<STALL_SPEC>`"]
pub type STALL = crate::Reg<stall::STALL_SPEC>;
#[doc = "usb1_ep_in stall register"]
pub mod stall;
#[doc = "idle (r) register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "usb1_ep_in idle register"]
pub mod idle;
#[doc = "have (r) register accessor: an alias for `Reg<HAVE_SPEC>`"]
pub type HAVE = crate::Reg<have::HAVE_SPEC>;
#[doc = "usb1_ep_in have register"]
pub mod have;
#[doc = "pend (r) register accessor: an alias for `Reg<PEND_SPEC>`"]
pub type PEND = crate::Reg<pend::PEND_SPEC>;
#[doc = "usb1_ep_in pend register"]
pub mod pend;
#[doc = "pid (rw) register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "usb1_ep_in pid register"]
pub mod pid;
#[doc = "ev_status (r) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "usb1_ep_in ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "usb1_ep_in ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "usb1_ep_in ev_enable register"]
pub mod ev_enable;
