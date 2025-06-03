#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tx_data: TX_DATA,
    _reserved1: [u8; 0x03],
    rx_data: RX_DATA,
    _reserved2: [u8; 0x03],
    tx_ready: TX_READY,
    _reserved3: [u8; 0x03],
    rx_avail: RX_AVAIL,
    _reserved4: [u8; 0x03],
    divisor: DIVISOR,
}
impl RegisterBlock {
    #[doc = "0x00 - valid to write to when tx_rdy is high, will trigger a transmit"]
    #[inline(always)]
    pub const fn tx_data(&self) -> &TX_DATA {
        &self.tx_data
    }
    #[doc = "0x04 - valid to read from when rx_avail is high, last received byte"]
    #[inline(always)]
    pub const fn rx_data(&self) -> &RX_DATA {
        &self.rx_data
    }
    #[doc = "0x08 - is '1' when 1-byte transmit buffer is empty"]
    #[inline(always)]
    pub const fn tx_ready(&self) -> &TX_READY {
        &self.tx_ready
    }
    #[doc = "0x0c - is '1' when 1-byte receive buffer is full; reset by a read from rx_data"]
    #[inline(always)]
    pub const fn rx_avail(&self) -> &RX_AVAIL {
        &self.rx_avail
    }
    #[doc = "0x10 - baud rate divider, defaults to init"]
    #[inline(always)]
    pub const fn divisor(&self) -> &DIVISOR {
        &self.divisor
    }
}
#[doc = "tx_data (rw) register accessor: valid to write to when tx_rdy is high, will trigger a transmit\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data`]
module"]
#[doc(alias = "tx_data")]
pub type TX_DATA = crate::Reg<tx_data::TX_DATA_SPEC>;
#[doc = "valid to write to when tx_rdy is high, will trigger a transmit"]
pub mod tx_data;
#[doc = "rx_data (rw) register accessor: valid to read from when rx_avail is high, last received byte\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data`]
module"]
#[doc(alias = "rx_data")]
pub type RX_DATA = crate::Reg<rx_data::RX_DATA_SPEC>;
#[doc = "valid to read from when rx_avail is high, last received byte"]
pub mod rx_data;
#[doc = "tx_ready (rw) register accessor: is '1' when 1-byte transmit buffer is empty\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ready`]
module"]
#[doc(alias = "tx_ready")]
pub type TX_READY = crate::Reg<tx_ready::TX_READY_SPEC>;
#[doc = "is '1' when 1-byte transmit buffer is empty"]
pub mod tx_ready;
#[doc = "rx_avail (rw) register accessor: is '1' when 1-byte receive buffer is full; reset by a read from rx_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_avail::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_avail::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_avail`]
module"]
#[doc(alias = "rx_avail")]
pub type RX_AVAIL = crate::Reg<rx_avail::RX_AVAIL_SPEC>;
#[doc = "is '1' when 1-byte receive buffer is full; reset by a read from rx_data"]
pub mod rx_avail;
#[doc = "divisor (rw) register accessor: baud rate divider, defaults to init\n\nYou can [`read`](crate::Reg::read) this register and get [`divisor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divisor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divisor`]
module"]
#[doc(alias = "divisor")]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "baud rate divider, defaults to init"]
pub mod divisor;
