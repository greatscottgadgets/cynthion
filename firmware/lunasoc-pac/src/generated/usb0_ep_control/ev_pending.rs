#[doc = "Register `ev_pending` reader"]
pub type R = crate::R<EV_PENDING_SPEC>;
#[doc = "Register `ev_pending` writer"]
pub type W = crate::W<EV_PENDING_SPEC>;
#[doc = "Field `pending` reader - usb0_ep_control pending register field"]
pub type PENDING_R = crate::BitReader;
#[doc = "Field `pending` writer - usb0_ep_control pending register field"]
pub type PENDING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - usb0_ep_control pending register field"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb0_ep_control pending register field"]
    #[inline(always)]
    #[must_use]
    pub fn pending(&mut self) -> PENDING_W<EV_PENDING_SPEC> {
        PENDING_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "usb0_ep_control ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_PENDING_SPEC;
impl crate::RegisterSpec for EV_PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EV_PENDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EV_PENDING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ev_pending to value 0"]
impl crate::Resettable for EV_PENDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
