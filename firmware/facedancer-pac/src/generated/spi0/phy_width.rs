#[doc = "Register `phy_width` reader"]
pub type R = crate::R<PHY_WIDTH_SPEC>;
#[doc = "Register `phy_width` writer"]
pub type W = crate::W<PHY_WIDTH_SPEC>;
#[doc = "Field `phy_width` reader - spi0 phy_width register field"]
pub type PHY_WIDTH_R = crate::FieldReader;
#[doc = "Field `phy_width` writer - spi0 phy_width register field"]
pub type PHY_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - spi0 phy_width register field"]
    #[inline(always)]
    pub fn phy_width(&self) -> PHY_WIDTH_R {
        PHY_WIDTH_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - spi0 phy_width register field"]
    #[inline(always)]
    #[must_use]
    pub fn phy_width(&mut self) -> PHY_WIDTH_W<PHY_WIDTH_SPEC> {
        PHY_WIDTH_W::new(self, 0)
    }
}
#[doc = "spi0 phy_width register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_WIDTH_SPEC;
impl crate::RegisterSpec for PHY_WIDTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_width::R`](R) reader structure"]
impl crate::Readable for PHY_WIDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_width::W`](W) writer structure"]
impl crate::Writable for PHY_WIDTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets phy_width to value 0"]
impl crate::Resettable for PHY_WIDTH_SPEC {
    const RESET_VALUE: u8 = 0;
}
