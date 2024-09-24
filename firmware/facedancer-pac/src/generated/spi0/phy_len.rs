#[doc = "Register `phy_len` reader"]
pub type R = crate::R<PHY_LEN_SPEC>;
#[doc = "Register `phy_len` writer"]
pub type W = crate::W<PHY_LEN_SPEC>;
#[doc = "Field `phy_len` reader - spi0 phy_len register field"]
pub type PHY_LEN_R = crate::FieldReader;
#[doc = "Field `phy_len` writer - spi0 phy_len register field"]
pub type PHY_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - spi0 phy_len register field"]
    #[inline(always)]
    pub fn phy_len(&self) -> PHY_LEN_R {
        PHY_LEN_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - spi0 phy_len register field"]
    #[inline(always)]
    #[must_use]
    pub fn phy_len(&mut self) -> PHY_LEN_W<PHY_LEN_SPEC> {
        PHY_LEN_W::new(self, 0)
    }
}
#[doc = "spi0 phy_len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_LEN_SPEC;
impl crate::RegisterSpec for PHY_LEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_len::R`](R) reader structure"]
impl crate::Readable for PHY_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_len::W`](W) writer structure"]
impl crate::Writable for PHY_LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets phy_len to value 0"]
impl crate::Resettable for PHY_LEN_SPEC {
    const RESET_VALUE: u8 = 0;
}
