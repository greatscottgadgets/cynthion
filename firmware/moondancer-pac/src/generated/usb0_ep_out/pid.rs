#[doc = "Register `pid` reader"]
pub type R = crate::R<PID_SPEC>;
#[doc = "Register `pid` writer"]
pub type W = crate::W<PID_SPEC>;
#[doc = "Field `toggle` writer - toggle field"]
pub type TOGGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - toggle field"]
    #[inline(always)]
    #[must_use]
    pub fn toggle(&mut self) -> TOGGLE_W<PID_SPEC> {
        TOGGLE_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<PID_SPEC> {
        _0_W::new(self, 1)
    }
}
#[doc = "Pid register toggle: Sets the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pid::W`](W) writer structure"]
impl crate::Writable for PID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets pid to value 0"]
impl crate::Resettable for PID_SPEC {
    const RESET_VALUE: u8 = 0;
}
