#[doc = "Register `ev_enable` reader"]
pub type R = crate::R<EV_ENABLE_SPEC>;
#[doc = "Register `ev_enable` writer"]
pub type W = crate::W<EV_ENABLE_SPEC>;
#[doc = "Field `mask` reader - mask field"]
pub type MASK_R = crate::BitReader;
#[doc = "Field `mask` writer - mask field"]
pub type MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - mask field"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mask field"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<EV_ENABLE_SPEC> {
        MASK_W::new(self, 0)
    }
}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_ENABLE_SPEC;
impl crate::RegisterSpec for EV_ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EV_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EV_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ev_enable to value 0"]
impl crate::Resettable for EV_ENABLE_SPEC {
    const RESET_VALUE: u8 = 0;
}
