#[doc = "Register `tx_ready` reader"]
pub type R = crate::R<TX_READY_SPEC>;
#[doc = "Register `tx_ready` writer"]
pub type W = crate::W<TX_READY_SPEC>;
#[doc = "Field `txe` reader - txe field"]
pub type TXE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - txe field"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_READY_SPEC;
impl crate::RegisterSpec for TX_READY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_ready::R`](R) reader structure"]
impl crate::Readable for TX_READY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ready::W`](W) writer structure"]
impl crate::Writable for TX_READY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets tx_ready to value 0"]
impl crate::Resettable for TX_READY_SPEC {
    const RESET_VALUE: u8 = 0;
}
