#[doc = "Register `ev_enable` reader"]
pub type R = crate::R<EV_ENABLE_SPEC>;
#[doc = "Register `ev_enable` writer"]
pub type W = crate::W<EV_ENABLE_SPEC>;
#[doc = "Field `enable` reader - gpiob enable register field"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - gpiob enable register field"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - gpiob enable register field"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - gpiob enable register field"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<EV_ENABLE_SPEC> {
        ENABLE_W::new(self, 0)
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
#[doc = "gpiob ev_enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_ENABLE_SPEC;
impl crate::RegisterSpec for EV_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EV_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EV_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ev_enable to value 0"]
impl crate::Resettable for EV_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
