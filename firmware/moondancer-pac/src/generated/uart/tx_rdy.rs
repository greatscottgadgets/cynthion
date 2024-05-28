#[doc = "Register `tx_rdy` reader"]
pub type R = crate::R<TX_RDY_SPEC>;
#[doc = "Field `tx_rdy` reader - uart tx_rdy register field"]
pub type TX_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - uart tx_rdy register field"]
    #[inline(always)]
    pub fn tx_rdy(&self) -> TX_RDY_R {
        TX_RDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "uart tx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rdy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_RDY_SPEC;
impl crate::RegisterSpec for TX_RDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_rdy::R`](R) reader structure"]
impl crate::Readable for TX_RDY_SPEC {}
#[doc = "`reset()` method sets tx_rdy to value 0"]
impl crate::Resettable for TX_RDY_SPEC {
    const RESET_VALUE: u32 = 0;
}
