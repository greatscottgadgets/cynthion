#[doc = "Register `rx_err` reader"]
pub type R = crate::R<RX_ERR_SPEC>;
#[doc = "Field `rx_err` reader - uart1 rx_err register field"]
pub type RX_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - uart1 rx_err register field"]
    #[inline(always)]
    pub fn rx_err(&self) -> RX_ERR_R {
        RX_ERR_R::new((self.bits & 7) as u8)
    }
}
#[doc = "uart1 rx_err register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ERR_SPEC;
impl crate::RegisterSpec for RX_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_err::R`](R) reader structure"]
impl crate::Readable for RX_ERR_SPEC {}
#[doc = "`reset()` method sets rx_err to value 0"]
impl crate::Resettable for RX_ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
