#[doc = "Register `rx_rdy` reader"]
pub type R = crate::R<RX_RDY_SPEC>;
#[doc = "Field `rx_rdy` reader - uart rx_rdy register field"]
pub type RX_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - uart rx_rdy register field"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RX_RDY_R {
        RX_RDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "uart rx_rdy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_rdy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_RDY_SPEC;
impl crate::RegisterSpec for RX_RDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_rdy::R`](R) reader structure"]
impl crate::Readable for RX_RDY_SPEC {}
#[doc = "`reset()` method sets rx_rdy to value 0"]
impl crate::Resettable for RX_RDY_SPEC {
    const RESET_VALUE: u32 = 0;
}
