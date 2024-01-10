#[doc = "Register `rx_data` reader"]
pub type R = crate::R<RX_DATA_SPEC>;
#[doc = "Field `rx_data` reader - uart rx_data register field"]
pub type RX_DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - uart rx_data register field"]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "uart rx_data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_SPEC;
impl crate::RegisterSpec for RX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data::R`](R) reader structure"]
impl crate::Readable for RX_DATA_SPEC {}
#[doc = "`reset()` method sets rx_data to value 0"]
impl crate::Resettable for RX_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
