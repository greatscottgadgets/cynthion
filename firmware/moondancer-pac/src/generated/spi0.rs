#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    phy: PHY,
    cs: CS,
    status: STATUS,
    _reserved3: [u8; 0x02],
    data: DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - PHY control register length : SPI transfer length in bits. width : SPI transfer bus width (1/2/4/8). mask : SPI DQ output enable mask."]
    #[inline(always)]
    pub const fn phy(&self) -> &PHY {
        &self.phy
    }
    #[doc = "0x04 - SPI chip select register select : SPI chip select signal."]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x05 - Status register rx_ready : RX FIFO contains data. tx_ready : TX FIFO ready to receive data."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08..0x10 - Data register rx : Read the next byte in the RX FIFO tx : Write the given byte to the TX FIFO"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
#[doc = "phy (rw) register accessor: PHY control register length : SPI transfer length in bits. width : SPI transfer bus width (1/2/4/8). mask : SPI DQ output enable mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy`]
module"]
#[doc(alias = "phy")]
pub type PHY = crate::Reg<phy::PHY_SPEC>;
#[doc = "PHY control register length : SPI transfer length in bits. width : SPI transfer bus width (1/2/4/8). mask : SPI DQ output enable mask."]
pub mod phy;
#[doc = "cs (rw) register accessor: SPI chip select register select : SPI chip select signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "cs")]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "SPI chip select register select : SPI chip select signal."]
pub mod cs;
#[doc = "status (rw) register accessor: Status register rx_ready : RX FIFO contains data. tx_ready : TX FIFO ready to receive data.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register rx_ready : RX FIFO contains data. tx_ready : TX FIFO ready to receive data."]
pub mod status;
#[doc = "data (rw) register accessor: Data register rx : Read the next byte in the RX FIFO tx : Write the given byte to the TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register rx : Read the next byte in the RX FIFO tx : Write the given byte to the TX FIFO"]
pub mod data;
