#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    phy: PHY,
    cs: CS,
    _reserved2: [u8; 0x03],
    rx: RX,
    tx: TX,
}
impl RegisterBlock {
    #[doc = "0x00 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn phy(&self) -> &PHY {
        &self.phy
    }
    #[doc = "0x04 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x08..0x10 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn rx(&self) -> &RX {
        &self.rx
    }
    #[doc = "0x10..0x18 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub const fn tx(&self) -> &TX {
        &self.tx
    }
}
#[doc = "phy (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy`]
module"]
#[doc(alias = "phy")]
pub type PHY = crate::Reg<phy::PHY_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod phy;
#[doc = "cs (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "cs")]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod cs;
#[doc = "rx (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`]
module"]
#[doc(alias = "rx")]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod rx;
#[doc = "tx (rw) register accessor: TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx`]
module"]
#[doc(alias = "tx")]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "TODO amaranth_soc/csr/reg.py:471"]
pub mod tx;
