#[doc = "Register `rx_data` reader"]
pub type R = crate::R<RX_DATA_SPEC>;
#[doc = "Register `rx_data` writer"]
pub type W = crate::W<RX_DATA_SPEC>;
#[doc = "Field `data` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_SPEC;
impl crate::RegisterSpec for RX_DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_data::R`](R) reader structure"]
impl crate::Readable for RX_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_data::W`](W) writer structure"]
impl crate::Writable for RX_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets rx_data to value 0"]
impl crate::Resettable for RX_DATA_SPEC {
    const RESET_VALUE: u8 = 0;
}
