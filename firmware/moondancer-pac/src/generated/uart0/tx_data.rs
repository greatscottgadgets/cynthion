#[doc = "Register `tx_data` reader"]
pub type R = crate::R<TX_DATA_SPEC>;
#[doc = "Register `tx_data` writer"]
pub type W = crate::W<TX_DATA_SPEC>;
#[doc = "Field `data` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TX_DATA_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_SPEC;
impl crate::RegisterSpec for TX_DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_data::R`](R) reader structure"]
impl crate::Readable for TX_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_data::W`](W) writer structure"]
impl crate::Writable for TX_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets tx_data to value 0"]
impl crate::Resettable for TX_DATA_SPEC {
    const RESET_VALUE: u8 = 0;
}
