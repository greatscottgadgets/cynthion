#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - uart divisor register"]
    pub divisor: DIVISOR,
    #[doc = "0x04 - uart rx_data register"]
    pub rx_data: RX_DATA,
    #[doc = "0x08 - uart rx_rdy register"]
    pub rx_rdy: RX_RDY,
    #[doc = "0x0c - uart rx_err register"]
    pub rx_err: RX_ERR,
    #[doc = "0x10 - uart tx_data register"]
    pub tx_data: TX_DATA,
    #[doc = "0x14 - uart tx_rdy register"]
    pub tx_rdy: TX_RDY,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - uart ev_status register"]
    pub ev_status: EV_STATUS,
    #[doc = "0x24 - uart ev_pending register"]
    pub ev_pending: EV_PENDING,
    #[doc = "0x28 - uart ev_enable register"]
    pub ev_enable: EV_ENABLE,
}
#[doc = "divisor (rw) register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "uart divisor register"]
pub mod divisor;
#[doc = "rx_data (r) register accessor: an alias for `Reg<RX_DATA_SPEC>`"]
pub type RX_DATA = crate::Reg<rx_data::RX_DATA_SPEC>;
#[doc = "uart rx_data register"]
pub mod rx_data;
#[doc = "rx_rdy (r) register accessor: an alias for `Reg<RX_RDY_SPEC>`"]
pub type RX_RDY = crate::Reg<rx_rdy::RX_RDY_SPEC>;
#[doc = "uart rx_rdy register"]
pub mod rx_rdy;
#[doc = "rx_err (r) register accessor: an alias for `Reg<RX_ERR_SPEC>`"]
pub type RX_ERR = crate::Reg<rx_err::RX_ERR_SPEC>;
#[doc = "uart rx_err register"]
pub mod rx_err;
#[doc = "tx_data (w) register accessor: an alias for `Reg<TX_DATA_SPEC>`"]
pub type TX_DATA = crate::Reg<tx_data::TX_DATA_SPEC>;
#[doc = "uart tx_data register"]
pub mod tx_data;
#[doc = "tx_rdy (r) register accessor: an alias for `Reg<TX_RDY_SPEC>`"]
pub type TX_RDY = crate::Reg<tx_rdy::TX_RDY_SPEC>;
#[doc = "uart tx_rdy register"]
pub mod tx_rdy;
#[doc = "ev_status (r) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "uart ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "uart ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "uart ev_enable register"]
pub mod ev_enable;
