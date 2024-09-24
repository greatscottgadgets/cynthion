#[doc = "Register `phy_mask` reader"]
pub type R = crate::R<PHY_MASK_SPEC>;
#[doc = "Register `phy_mask` writer"]
pub type W = crate::W<PHY_MASK_SPEC>;
#[doc = "Field `phy_mask` reader - spi0 phy_mask register field"]
pub type PHY_MASK_R = crate::FieldReader;
#[doc = "Field `phy_mask` writer - spi0 phy_mask register field"]
pub type PHY_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - spi0 phy_mask register field"]
    #[inline(always)]
    pub fn phy_mask(&self) -> PHY_MASK_R {
        PHY_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - spi0 phy_mask register field"]
    #[inline(always)]
    #[must_use]
    pub fn phy_mask(&mut self) -> PHY_MASK_W<PHY_MASK_SPEC> {
        PHY_MASK_W::new(self, 0)
    }
}
#[doc = "spi0 phy_mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_MASK_SPEC;
impl crate::RegisterSpec for PHY_MASK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_mask::R`](R) reader structure"]
impl crate::Readable for PHY_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_mask::W`](W) writer structure"]
impl crate::Writable for PHY_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets phy_mask to value 0"]
impl crate::Resettable for PHY_MASK_SPEC {
    const RESET_VALUE: u8 = 0;
}
