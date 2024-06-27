#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    phy_len: PHY_LEN,
    phy_width: PHY_WIDTH,
    phy_mask: PHY_MASK,
    cs: CS,
    rxtx: RXTX,
    tx_rdy: TX_RDY,
    rx_rdy: RX_RDY,
}
impl RegisterBlock {
    #[doc = "0x00 - spi0 phy_len register"]
    #[inline(always)]
    pub const fn phy_len(&self) -> &PHY_LEN {
        &self.phy_len
    }
    #[doc = "0x01 - spi0 phy_width register"]
    #[inline(always)]
    pub const fn phy_width(&self) -> &PHY_WIDTH {
        &self.phy_width
    }
    #[doc = "0x02 - spi0 phy_mask register"]
    #[inline(always)]
    pub const fn phy_mask(&self) -> &PHY_MASK {
        &self.phy_mask
    }
    #[doc = "0x03 - spi0 cs register"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x04 - spi0 rxtx register"]
    #[inline(always)]
    pub const fn rxtx(&self) -> &RXTX {
        &self.rxtx
    }
    #[doc = "0x08 - spi0 tx_rdy register"]
    #[inline(always)]
    pub const fn tx_rdy(&self) -> &TX_RDY {
        &self.tx_rdy
    }
    #[doc = "0x09 - spi0 rx_rdy register"]
    #[inline(always)]
    pub const fn rx_rdy(&self) -> &RX_RDY {
        &self.rx_rdy
    }
}
#[doc = "phy_len (rw) register accessor: spi0 phy_len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_len`]
module"]
#[doc(alias = "phy_len")]
pub type PHY_LEN = crate::Reg<phy_len::PHY_LEN_SPEC>;
#[doc = "spi0 phy_len register"]
pub mod phy_len;
#[doc = "phy_width (rw) register accessor: spi0 phy_width register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_width`]
module"]
#[doc(alias = "phy_width")]
pub type PHY_WIDTH = crate::Reg<phy_width::PHY_WIDTH_SPEC>;
#[doc = "spi0 phy_width register"]
pub mod phy_width;
#[doc = "phy_mask (rw) register accessor: spi0 phy_mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_mask`]
module"]
#[doc(alias = "phy_mask")]
pub type PHY_MASK = crate::Reg<phy_mask::PHY_MASK_SPEC>;
#[doc = "spi0 phy_mask register"]
pub mod phy_mask;
#[doc = "cs (rw) register accessor: spi0 cs register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "cs")]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "spi0 cs register"]
pub mod cs;
#[doc = "rxtx (rw) register accessor: spi0 rxtx register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtx`]
module"]
#[doc(alias = "rxtx")]
pub type RXTX = crate::Reg<rxtx::RXTX_SPEC>;
#[doc = "spi0 rxtx register"]
pub mod rxtx;
#[doc = "tx_rdy (r) register accessor: spi0 tx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rdy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_rdy`]
module"]
#[doc(alias = "tx_rdy")]
pub type TX_RDY = crate::Reg<tx_rdy::TX_RDY_SPEC>;
#[doc = "spi0 tx_rdy register"]
pub mod tx_rdy;
#[doc = "rx_rdy (r) register accessor: spi0 rx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_rdy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_rdy`]
module"]
#[doc(alias = "rx_rdy")]
pub type RX_RDY = crate::Reg<rx_rdy::RX_RDY_SPEC>;
#[doc = "spi0 rx_rdy register"]
pub mod rx_rdy;
