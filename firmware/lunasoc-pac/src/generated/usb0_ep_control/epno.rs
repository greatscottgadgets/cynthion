#[doc = "Register `epno` reader"]
pub type R = crate::R<EPNO_SPEC>;
#[doc = "Field `epno` reader - The endpoint number associated with the most recently captured SETUP packet."]
pub type EPNO_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The endpoint number associated with the most recently captured SETUP packet."]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "usb0_ep_control epno register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epno::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPNO_SPEC;
impl crate::RegisterSpec for EPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epno::R`](R) reader structure"]
impl crate::Readable for EPNO_SPEC {}
#[doc = "`reset()` method sets epno to value 0"]
impl crate::Resettable for EPNO_SPEC {
    const RESET_VALUE: u32 = 0;
}
