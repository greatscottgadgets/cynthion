#[doc = "Register `idle` reader"]
pub type R = crate::R<IDLE_SPEC>;
#[doc = "Field `idle` reader - This value is `1` if no packet is actively being transmitted."]
pub type IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This value is `1` if no packet is actively being transmitted."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "usb1_ep_in idle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLE_SPEC;
impl crate::RegisterSpec for IDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle::R`](R) reader structure"]
impl crate::Readable for IDLE_SPEC {}
#[doc = "`reset()` method sets idle to value 0"]
impl crate::Resettable for IDLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
