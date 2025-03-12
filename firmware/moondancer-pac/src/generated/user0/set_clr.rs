#[doc = "Register `SetClr` reader"]
pub type R = crate::R<SET_CLR_SPEC>;
#[doc = "Register `SetClr` writer"]
pub type W = crate::W<SET_CLR_SPEC>;
#[doc = "Field `pin_0_set` writer - pin_0_set field"]
pub type PIN_0_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_0_clr` writer - pin_0_clr field"]
pub type PIN_0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - pin_0_set field"]
    #[inline(always)]
    pub fn pin_0_set(&mut self) -> PIN_0_SET_W<SET_CLR_SPEC> {
        PIN_0_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - pin_0_clr field"]
    #[inline(always)]
    pub fn pin_0_clr(&mut self) -> PIN_0_CLR_W<SET_CLR_SPEC> {
        PIN_0_CLR_W::new(self, 1)
    }
}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`set_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_CLR_SPEC;
impl crate::RegisterSpec for SET_CLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`set_clr::R`](R) reader structure"]
impl crate::Readable for SET_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set_clr::W`](W) writer structure"]
impl crate::Writable for SET_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SetClr to value 0"]
impl crate::Resettable for SET_CLR_SPEC {
    const RESET_VALUE: u8 = 0;
}
