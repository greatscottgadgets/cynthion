#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    divisor: DIVISOR,
    rx_data: RX_DATA,
    rx_rdy: RX_RDY,
    rx_err: RX_ERR,
    tx_data: TX_DATA,
    tx_rdy: TX_RDY,
    _reserved6: [u8; 0x08],
    ev_status: EV_STATUS,
    ev_pending: EV_PENDING,
    ev_enable: EV_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00 - uart divisor register"]
    #[inline(always)]
    pub const fn divisor(&self) -> &DIVISOR {
        &self.divisor
    }
    #[doc = "0x04 - uart rx_data register"]
    #[inline(always)]
    pub const fn rx_data(&self) -> &RX_DATA {
        &self.rx_data
    }
    #[doc = "0x08 - uart rx_rdy register"]
    #[inline(always)]
    pub const fn rx_rdy(&self) -> &RX_RDY {
        &self.rx_rdy
    }
    #[doc = "0x0c - uart rx_err register"]
    #[inline(always)]
    pub const fn rx_err(&self) -> &RX_ERR {
        &self.rx_err
    }
    #[doc = "0x10 - uart tx_data register"]
    #[inline(always)]
    pub const fn tx_data(&self) -> &TX_DATA {
        &self.tx_data
    }
    #[doc = "0x14 - uart tx_rdy register"]
    #[inline(always)]
    pub const fn tx_rdy(&self) -> &TX_RDY {
        &self.tx_rdy
    }
    #[doc = "0x20 - uart ev_status register"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EV_STATUS {
        &self.ev_status
    }
    #[doc = "0x24 - uart ev_pending register"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
    #[doc = "0x28 - uart ev_enable register"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
}
#[doc = "divisor (rw) register accessor: uart divisor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divisor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divisor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divisor`]
module"]
#[doc(alias = "divisor")]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "uart divisor register"]
pub mod divisor;
#[doc = "rx_data (r) register accessor: uart rx_data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data`]
module"]
#[doc(alias = "rx_data")]
pub type RX_DATA = crate::Reg<rx_data::RX_DATA_SPEC>;
#[doc = "uart rx_data register"]
pub mod rx_data;
#[doc = "rx_rdy (r) register accessor: uart rx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_rdy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_rdy`]
module"]
#[doc(alias = "rx_rdy")]
pub type RX_RDY = crate::Reg<rx_rdy::RX_RDY_SPEC>;
#[doc = "uart rx_rdy register"]
pub mod rx_rdy;
#[doc = "rx_err (r) register accessor: uart rx_err register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err`]
module"]
#[doc(alias = "rx_err")]
pub type RX_ERR = crate::Reg<rx_err::RX_ERR_SPEC>;
#[doc = "uart rx_err register"]
pub mod rx_err;
#[doc = "tx_data (w) register accessor: uart tx_data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data`]
module"]
#[doc(alias = "tx_data")]
pub type TX_DATA = crate::Reg<tx_data::TX_DATA_SPEC>;
#[doc = "uart tx_data register"]
pub mod tx_data;
#[doc = "tx_rdy (r) register accessor: uart tx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rdy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_rdy`]
module"]
#[doc(alias = "tx_rdy")]
pub type TX_RDY = crate::Reg<tx_rdy::TX_RDY_SPEC>;
#[doc = "uart tx_rdy register"]
pub mod tx_rdy;
#[doc = "ev_status (r) register accessor: uart ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`]
module"]
#[doc(alias = "ev_status")]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "uart ev_status register"]
pub mod ev_status;
#[doc = "ev_pending (rw) register accessor: uart ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "uart ev_pending register"]
pub mod ev_pending;
#[doc = "ev_enable (rw) register accessor: uart ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "uart ev_enable register"]
pub mod ev_enable;
