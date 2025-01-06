#[doc = "Register `counter` reader"]
pub type R = crate::R<COUNTER_SPEC>;
#[doc = "Register `counter` writer"]
pub type W = crate::W<COUNTER_SPEC>;
#[doc = "Field `value` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for COUNTER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets counter to value 0"]
impl crate::Resettable for COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
