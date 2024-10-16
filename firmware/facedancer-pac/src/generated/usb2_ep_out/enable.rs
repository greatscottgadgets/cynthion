#[doc = "Register `enable` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `enabled` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<ENABLE_SPEC> {
        ENABLED_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<ENABLE_SPEC> {
        _0_W::new(self, 1)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u8 = 0;
}
