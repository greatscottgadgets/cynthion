#[doc = "Register `phy` reader"]
pub type R = crate::R<PHY_SPEC>;
#[doc = "Register `phy` writer"]
pub type W = crate::W<PHY_SPEC>;
#[doc = "Field `length` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type LENGTH_R = crate::FieldReader;
#[doc = "Field `length` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `width` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `width` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mask` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `mask` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:17 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<PHY_SPEC> {
        LENGTH_W::new(self, 0)
    }
    #[doc = "Bits 6:9 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<PHY_SPEC> {
        WIDTH_W::new(self, 6)
    }
    #[doc = "Bits 10:17 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<PHY_SPEC> {
        MASK_W::new(self, 10)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_SPEC;
impl crate::RegisterSpec for PHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy::R`](R) reader structure"]
impl crate::Readable for PHY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy::W`](W) writer structure"]
impl crate::Writable for PHY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phy to value 0"]
impl crate::Resettable for PHY_SPEC {
    const RESET_VALUE: u32 = 0;
}
