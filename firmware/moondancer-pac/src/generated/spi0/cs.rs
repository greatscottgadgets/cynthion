#[doc = "Register `cs` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `cs` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `select` writer - select field"]
pub type SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - select field"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<CS_SPEC> {
        SELECT_W::new(self, 0)
    }
}
#[doc = "SPI chip select register select : SPI chip select signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
