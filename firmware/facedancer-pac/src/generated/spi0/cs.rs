#[doc = "Register `cs` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `cs` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `cs` reader - spi0 cs register field"]
pub type CS_R = crate::BitReader;
#[doc = "Field `cs` writer - spi0 cs register field"]
pub type CS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - spi0 cs register field"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - spi0 cs register field"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<CS_SPEC> {
        CS_W::new(self, 0)
    }
}
#[doc = "spi0 cs register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets cs to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: u8 = 0;
}
