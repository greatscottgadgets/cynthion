#[doc = "Register `Mode` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `Mode` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `pin_0` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_0_R = crate::FieldReader;
#[doc = "Field `pin_0` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_1` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_1_R = crate::FieldReader;
#[doc = "Field `pin_1` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_2` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_2_R = crate::FieldReader;
#[doc = "Field `pin_2` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_3` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_3_R = crate::FieldReader;
#[doc = "Field `pin_3` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_4` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_4_R = crate::FieldReader;
#[doc = "Field `pin_4` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_5` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_5_R = crate::FieldReader;
#[doc = "Field `pin_5` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_6` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_6_R = crate::FieldReader;
#[doc = "Field `pin_6` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_7` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_7_R = crate::FieldReader;
#[doc = "Field `pin_7` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_1(&self) -> PIN_1_R {
        PIN_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_2(&self) -> PIN_2_R {
        PIN_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_3(&self) -> PIN_3_R {
        PIN_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_4(&self) -> PIN_4_R {
        PIN_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_5(&self) -> PIN_5_R {
        PIN_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_6(&self) -> PIN_6_R {
        PIN_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_7(&self) -> PIN_7_R {
        PIN_7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_0(&mut self) -> PIN_0_W<MODE_SPEC> {
        PIN_0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_1(&mut self) -> PIN_1_W<MODE_SPEC> {
        PIN_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_2(&mut self) -> PIN_2_W<MODE_SPEC> {
        PIN_2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_3(&mut self) -> PIN_3_W<MODE_SPEC> {
        PIN_3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_4(&mut self) -> PIN_4_W<MODE_SPEC> {
        PIN_4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_5(&mut self) -> PIN_5_W<MODE_SPEC> {
        PIN_5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_6(&mut self) -> PIN_6_W<MODE_SPEC> {
        PIN_6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn pin_7(&mut self) -> PIN_7_W<MODE_SPEC> {
        PIN_7_W::new(self, 14)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets Mode to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u16 = 0;
}
