#[doc = "Register `rx_avail` reader"]
pub type R = crate::R<RX_AVAIL_SPEC>;
#[doc = "Register `rx_avail` writer"]
pub type W = crate::W<RX_AVAIL_SPEC>;
#[doc = "Field `rxe` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type RXE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_avail::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_avail::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_AVAIL_SPEC;
impl crate::RegisterSpec for RX_AVAIL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_avail::R`](R) reader structure"]
impl crate::Readable for RX_AVAIL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_avail::W`](W) writer structure"]
impl crate::Writable for RX_AVAIL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets rx_avail to value 0"]
impl crate::Resettable for RX_AVAIL_SPEC {
    const RESET_VALUE: u8 = 0;
}
