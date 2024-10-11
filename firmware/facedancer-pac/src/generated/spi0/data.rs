#[doc = "Register `data` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Register `data` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `rx` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type RX_R = crate::FieldReader<u32>;
#[doc = "Field `tx` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 32:63 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<DATA_SPEC> {
        TX_W::new(self, 32)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets data to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u64 = 0;
}
