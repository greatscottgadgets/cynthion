#[doc = "Register `endpoint` reader"]
pub type R = crate::R<ENDPOINT_SPEC>;
#[doc = "Register `endpoint` writer"]
pub type W = crate::W<ENDPOINT_SPEC>;
#[doc = "Field `number` reader - number field"]
pub type NUMBER_R = crate::FieldReader;
#[doc = "Field `number` writer - number field"]
pub type NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - number field"]
    #[inline(always)]
    pub fn number(&self) -> NUMBER_R {
        NUMBER_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - number field"]
    #[inline(always)]
    pub fn number(&mut self) -> NUMBER_W<ENDPOINT_SPEC> {
        NUMBER_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0_W<ENDPOINT_SPEC> {
        _0_W::new(self, 4)
    }
}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`endpoint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpoint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPOINT_SPEC;
impl crate::RegisterSpec for ENDPOINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`endpoint::R`](R) reader structure"]
impl crate::Readable for ENDPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endpoint::W`](W) writer structure"]
impl crate::Writable for ENDPOINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets endpoint to value 0"]
impl crate::Resettable for ENDPOINT_SPEC {
    const RESET_VALUE: u8 = 0;
}
