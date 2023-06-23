#[doc = "Register `rx_rdy` reader"]
pub struct R(crate::R<RX_RDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_rdy` reader - uart rx_rdy register field"]
pub type RX_RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - uart rx_rdy register field"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RX_RDY_R {
        RX_RDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "uart rx_rdy register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_rdy](index.html) module"]
pub struct RX_RDY_SPEC;
impl crate::RegisterSpec for RX_RDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_rdy::R](R) reader structure"]
impl crate::Readable for RX_RDY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx_rdy to value 0"]
impl crate::Resettable for RX_RDY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
