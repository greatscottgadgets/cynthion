#[doc = "Register `tx_data` writer"]
pub type W = crate::W<TX_DATA_SPEC>;
#[doc = "Field `tx_data` writer - uart1 tx_data register field"]
pub type TX_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - uart1 tx_data register field"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<TX_DATA_SPEC> {
        TX_DATA_W::new(self, 0)
    }
}
#[doc = "uart1 tx_data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_SPEC;
impl crate::RegisterSpec for TX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data::W`](W) writer structure"]
impl crate::Writable for TX_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_data to value 0"]
impl crate::Resettable for TX_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
