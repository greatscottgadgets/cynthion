#[doc = "Register `epno` reader"]
pub struct R(crate::R<EPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `epno` reader - The number of the endpoint associated with the current SETUP packet."]
pub type EPNO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - The number of the endpoint associated with the current SETUP packet."]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "usb1_ep_control epno register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epno](index.html) module"]
pub struct EPNO_SPEC;
impl crate::RegisterSpec for EPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epno::R](R) reader structure"]
impl crate::Readable for EPNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets epno to value 0"]
impl crate::Resettable for EPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
