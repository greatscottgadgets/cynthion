#[doc = "Register `tx_rdy` reader"]
pub struct R(crate::R<TX_RDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tx_rdy` reader - uart tx_rdy register field"]
pub type TX_RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - uart tx_rdy register field"]
    #[inline(always)]
    pub fn tx_rdy(&self) -> TX_RDY_R {
        TX_RDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "uart tx_rdy register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rdy](index.html) module"]
pub struct TX_RDY_SPEC;
impl crate::RegisterSpec for TX_RDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rdy::R](R) reader structure"]
impl crate::Readable for TX_RDY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx_rdy to value 0"]
impl crate::Resettable for TX_RDY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
