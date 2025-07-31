#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `rx_ready` reader - rx_ready field"]
pub type RX_READY_R = crate::BitReader;
#[doc = "Field `tx_ready` reader - tx_ready field"]
pub type TX_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - rx_ready field"]
    #[inline(always)]
    pub fn rx_ready(&self) -> RX_READY_R {
        RX_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx_ready field"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Status register rx_ready : RX FIFO contains data. tx_ready : TX FIFO ready to receive data.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u8 = 0;
}
