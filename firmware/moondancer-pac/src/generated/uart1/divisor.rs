#[doc = "Register `divisor` reader"]
pub type R = crate::R<DIVISOR_SPEC>;
#[doc = "Register `divisor` writer"]
pub type W = crate::W<DIVISOR_SPEC>;
#[doc = "Field `div` reader - div field"]
pub type DIV_R = crate::FieldReader<u32>;
#[doc = "Field `div` writer - div field"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - div field"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - div field"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIVISOR_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "baud rate divider, defaults to init\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divisor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divisor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divisor::R`](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divisor::W`](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets divisor to value 0"]
impl crate::Resettable for DIVISOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
