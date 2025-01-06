#[doc = "Register `SetClr` reader"]
pub type R = crate::R<SET_CLR_SPEC>;
#[doc = "Register `SetClr` writer"]
pub type W = crate::W<SET_CLR_SPEC>;
#[doc = "Field `pin_0_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_0_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_0_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_1_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_1_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_1_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_1_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_2_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_2_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_2_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_2_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_3_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_3_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_3_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_3_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_4_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_4_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_4_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_4_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_5_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_5_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_5_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_5_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_6_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_6_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_6_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_6_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_7_set` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_7_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_7_clr` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_7_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_0_set(&mut self) -> PIN_0_SET_W<SET_CLR_SPEC> {
        PIN_0_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_0_clr(&mut self) -> PIN_0_CLR_W<SET_CLR_SPEC> {
        PIN_0_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_1_set(&mut self) -> PIN_1_SET_W<SET_CLR_SPEC> {
        PIN_1_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_1_clr(&mut self) -> PIN_1_CLR_W<SET_CLR_SPEC> {
        PIN_1_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_2_set(&mut self) -> PIN_2_SET_W<SET_CLR_SPEC> {
        PIN_2_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_2_clr(&mut self) -> PIN_2_CLR_W<SET_CLR_SPEC> {
        PIN_2_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_3_set(&mut self) -> PIN_3_SET_W<SET_CLR_SPEC> {
        PIN_3_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_3_clr(&mut self) -> PIN_3_CLR_W<SET_CLR_SPEC> {
        PIN_3_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_4_set(&mut self) -> PIN_4_SET_W<SET_CLR_SPEC> {
        PIN_4_SET_W::new(self, 8)
    }
    #[doc = "Bit 9 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_4_clr(&mut self) -> PIN_4_CLR_W<SET_CLR_SPEC> {
        PIN_4_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_5_set(&mut self) -> PIN_5_SET_W<SET_CLR_SPEC> {
        PIN_5_SET_W::new(self, 10)
    }
    #[doc = "Bit 11 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_5_clr(&mut self) -> PIN_5_CLR_W<SET_CLR_SPEC> {
        PIN_5_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_6_set(&mut self) -> PIN_6_SET_W<SET_CLR_SPEC> {
        PIN_6_SET_W::new(self, 12)
    }
    #[doc = "Bit 13 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_6_clr(&mut self) -> PIN_6_CLR_W<SET_CLR_SPEC> {
        PIN_6_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_7_set(&mut self) -> PIN_7_SET_W<SET_CLR_SPEC> {
        PIN_7_SET_W::new(self, 14)
    }
    #[doc = "Bit 15 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_7_clr(&mut self) -> PIN_7_CLR_W<SET_CLR_SPEC> {
        PIN_7_CLR_W::new(self, 15)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_CLR_SPEC;
impl crate::RegisterSpec for SET_CLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`set_clr::R`](R) reader structure"]
impl crate::Readable for SET_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set_clr::W`](W) writer structure"]
impl crate::Writable for SET_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SetClr to value 0"]
impl crate::Resettable for SET_CLR_SPEC {
    const RESET_VALUE: u16 = 0;
}
