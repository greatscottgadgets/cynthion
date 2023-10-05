#[doc = "Register `rx_err` reader"]
pub struct R(crate::R<RX_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_err` reader - uart rx_err register field"]
pub type RX_ERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - uart rx_err register field"]
    #[inline(always)]
    pub fn rx_err(&self) -> RX_ERR_R {
        RX_ERR_R::new((self.bits & 7) as u8)
    }
}
#[doc = "uart rx_err register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err](index.html) module"]
pub struct RX_ERR_SPEC;
impl crate::RegisterSpec for RX_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_err::R](R) reader structure"]
impl crate::Readable for RX_ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx_err to value 0"]
impl crate::Resettable for RX_ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
