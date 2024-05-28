#[doc = "Register `pid` reader"]
pub type R = crate::R<PID_SPEC>;
#[doc = "Register `pid` writer"]
pub type W = crate::W<PID_SPEC>;
#[doc = "Field `pid` reader - Contains the current PID toggle bit for the given endpoint."]
pub type PID_R = crate::BitReader;
#[doc = "Field `pid` writer - Contains the current PID toggle bit for the given endpoint."]
pub type PID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<PID_SPEC> {
        PID_W::new(self, 0)
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
#[doc = "usb0_ep_out pid register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pid::W`](W) writer structure"]
impl crate::Writable for PID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pid to value 0"]
impl crate::Resettable for PID_SPEC {
    const RESET_VALUE: u32 = 0;
}
