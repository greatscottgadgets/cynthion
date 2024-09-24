#[doc = "Register `pend` reader"]
pub type R = crate::R<PEND_SPEC>;
#[doc = "Field `pend` reader - `1` iff an interrupt is pending"]
pub type PEND_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - `1` iff an interrupt is pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new((self.bits & 1) != 0)
    }
}
#[doc = "usb0_ep_in pend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEND_SPEC;
impl crate::RegisterSpec for PEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pend::R`](R) reader structure"]
impl crate::Readable for PEND_SPEC {}
#[doc = "`reset()` method sets pend to value 0"]
impl crate::Resettable for PEND_SPEC {
    const RESET_VALUE: u32 = 0;
}
