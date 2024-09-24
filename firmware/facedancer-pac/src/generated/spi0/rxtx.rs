#[doc = "Register `rxtx` reader"]
pub type R = crate::R<RXTX_SPEC>;
#[doc = "Register `rxtx` writer"]
pub type W = crate::W<RXTX_SPEC>;
#[doc = "Field `rxtx` reader - spi0 rxtx register field"]
pub type RXTX_R = crate::FieldReader<u32>;
#[doc = "Field `rxtx` writer - spi0 rxtx register field"]
pub type RXTX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - spi0 rxtx register field"]
    #[inline(always)]
    pub fn rxtx(&self) -> RXTX_R {
        RXTX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - spi0 rxtx register field"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx(&mut self) -> RXTX_W<RXTX_SPEC> {
        RXTX_W::new(self, 0)
    }
}
#[doc = "spi0 rxtx register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTX_SPEC;
impl crate::RegisterSpec for RXTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtx::R`](R) reader structure"]
impl crate::Readable for RXTX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxtx::W`](W) writer structure"]
impl crate::Writable for RXTX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxtx to value 0"]
impl crate::Resettable for RXTX_SPEC {
    const RESET_VALUE: u32 = 0;
}
