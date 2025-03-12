#[doc = "Register `divisor` reader"]
pub type R = crate::R<DIVISOR_SPEC>;
#[doc = "Register `divisor` writer"]
pub type W = crate::W<DIVISOR_SPEC>;
#[doc = "Field `div` reader - div field"]
pub type DIV_R = crate::FieldReader<u32>;
#[doc = "Field `div` writer - div field"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - div field"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - div field"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<DIVISOR_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`divisor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divisor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divisor::R`](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divisor::W`](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets divisor to value 0"]
impl crate::Resettable for DIVISOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
