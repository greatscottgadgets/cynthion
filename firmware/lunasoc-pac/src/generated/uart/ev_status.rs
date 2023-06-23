#[doc = "Register `ev_status` reader"]
pub struct R(crate::R<EV_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `status` reader - uart status register field"]
pub type STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - uart status register field"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 7) as u8)
    }
}
#[doc = "uart ev_status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_status](index.html) module"]
pub struct EV_STATUS_SPEC;
impl crate::RegisterSpec for EV_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_status::R](R) reader structure"]
impl crate::Readable for EV_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ev_status to value 0"]
impl crate::Resettable for EV_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
