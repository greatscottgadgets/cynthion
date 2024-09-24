#[doc = "Register `have` reader"]
pub type R = crate::R<HAVE_SPEC>;
#[doc = "Field `have` reader - This value is `1` if data is present in the transmit FIFO."]
pub type HAVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This value is `1` if data is present in the transmit FIFO."]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "usb2_ep_in have register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`have::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAVE_SPEC;
impl crate::RegisterSpec for HAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`have::R`](R) reader structure"]
impl crate::Readable for HAVE_SPEC {}
#[doc = "`reset()` method sets have to value 0"]
impl crate::Resettable for HAVE_SPEC {
    const RESET_VALUE: u32 = 0;
}
