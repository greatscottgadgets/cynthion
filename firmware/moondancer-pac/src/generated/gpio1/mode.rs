#[doc = "Register `Mode` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `Mode` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `pin_0` reader - pin_0 field"]
pub type PIN_0_R = crate::FieldReader;
#[doc = "Field `pin_0` writer - pin_0 field"]
pub type PIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_1` reader - pin_1 field"]
pub type PIN_1_R = crate::FieldReader;
#[doc = "Field `pin_1` writer - pin_1 field"]
pub type PIN_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_2` reader - pin_2 field"]
pub type PIN_2_R = crate::FieldReader;
#[doc = "Field `pin_2` writer - pin_2 field"]
pub type PIN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_3` reader - pin_3 field"]
pub type PIN_3_R = crate::FieldReader;
#[doc = "Field `pin_3` writer - pin_3 field"]
pub type PIN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_4` reader - pin_4 field"]
pub type PIN_4_R = crate::FieldReader;
#[doc = "Field `pin_4` writer - pin_4 field"]
pub type PIN_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_5` reader - pin_5 field"]
pub type PIN_5_R = crate::FieldReader;
#[doc = "Field `pin_5` writer - pin_5 field"]
pub type PIN_5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_6` reader - pin_6 field"]
pub type PIN_6_R = crate::FieldReader;
#[doc = "Field `pin_6` writer - pin_6 field"]
pub type PIN_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_7` reader - pin_7 field"]
pub type PIN_7_R = crate::FieldReader;
#[doc = "Field `pin_7` writer - pin_7 field"]
pub type PIN_7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - pin_1 field"]
    #[inline(always)]
    pub fn pin_1(&self) -> PIN_1_R {
        PIN_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - pin_2 field"]
    #[inline(always)]
    pub fn pin_2(&self) -> PIN_2_R {
        PIN_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - pin_3 field"]
    #[inline(always)]
    pub fn pin_3(&self) -> PIN_3_R {
        PIN_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - pin_4 field"]
    #[inline(always)]
    pub fn pin_4(&self) -> PIN_4_R {
        PIN_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - pin_5 field"]
    #[inline(always)]
    pub fn pin_5(&self) -> PIN_5_R {
        PIN_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - pin_6 field"]
    #[inline(always)]
    pub fn pin_6(&self) -> PIN_6_R {
        PIN_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - pin_7 field"]
    #[inline(always)]
    pub fn pin_7(&self) -> PIN_7_R {
        PIN_7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&mut self) -> PIN_0_W<MODE_SPEC> {
        PIN_0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - pin_1 field"]
    #[inline(always)]
    pub fn pin_1(&mut self) -> PIN_1_W<MODE_SPEC> {
        PIN_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - pin_2 field"]
    #[inline(always)]
    pub fn pin_2(&mut self) -> PIN_2_W<MODE_SPEC> {
        PIN_2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - pin_3 field"]
    #[inline(always)]
    pub fn pin_3(&mut self) -> PIN_3_W<MODE_SPEC> {
        PIN_3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - pin_4 field"]
    #[inline(always)]
    pub fn pin_4(&mut self) -> PIN_4_W<MODE_SPEC> {
        PIN_4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - pin_5 field"]
    #[inline(always)]
    pub fn pin_5(&mut self) -> PIN_5_W<MODE_SPEC> {
        PIN_5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - pin_6 field"]
    #[inline(always)]
    pub fn pin_6(&mut self) -> PIN_6_W<MODE_SPEC> {
        PIN_6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - pin_7 field"]
    #[inline(always)]
    pub fn pin_7(&mut self) -> PIN_7_W<MODE_SPEC> {
        PIN_7_W::new(self, 14)
    }
}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
