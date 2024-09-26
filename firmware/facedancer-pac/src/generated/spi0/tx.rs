#[doc = "Register `tx` reader"]
pub type R = crate::R<TX_SPEC>;
#[doc = "Register `tx` writer"]
pub type W = crate::W<TX_SPEC>;
#[doc = "Field `data` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[doc = "Field `ready` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 32 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 32) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TX_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`tx::R`](R) reader structure"]
impl crate::Readable for TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx::W`](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets tx to value 0"]
impl crate::Resettable for TX_SPEC {
    const RESET_VALUE: u64 = 0;
}
