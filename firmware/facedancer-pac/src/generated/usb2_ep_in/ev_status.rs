#[doc = "Register `ev_status` reader"]
pub type R = crate::R<EV_STATUS_SPEC>;
#[doc = "Field `status` reader - usb2_ep_in status register field"]
pub type STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - usb2_ep_in status register field"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "usb2_ep_in ev_status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_STATUS_SPEC;
impl crate::RegisterSpec for EV_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EV_STATUS_SPEC {}
#[doc = "`reset()` method sets ev_status to value 0"]
impl crate::Resettable for EV_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
