#[doc = "Register `ev_pending` reader"]
pub type R = crate::R<EV_PENDING_SPEC>;
#[doc = "Register `ev_pending` writer"]
pub type W = crate::W<EV_PENDING_SPEC>;
#[doc = "Field `mask` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type MASK_R = crate::BitReader;
#[doc = "Field `mask` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<EV_PENDING_SPEC> {
        MASK_W::new(self, 0)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_PENDING_SPEC;
impl crate::RegisterSpec for EV_PENDING_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EV_PENDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EV_PENDING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ev_pending to value 0"]
impl crate::Resettable for EV_PENDING_SPEC {
    const RESET_VALUE: u8 = 0;
}
