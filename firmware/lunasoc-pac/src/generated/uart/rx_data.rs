#[doc = "Register `rx_data` reader"]
pub struct R(crate::R<RX_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_data` reader - uart rx_data register field"]
pub type RX_DATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - uart rx_data register field"]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "uart rx_data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data](index.html) module"]
pub struct RX_DATA_SPEC;
impl crate::RegisterSpec for RX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data::R](R) reader structure"]
impl crate::Readable for RX_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx_data to value 0"]
impl crate::Resettable for RX_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
