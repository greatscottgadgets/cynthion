#[doc = "Register `prime` writer"]
pub type W = crate::W<PRIME_SPEC>;
#[doc = "Field `prime` writer - Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
pub type PRIME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
    #[inline(always)]
    #[must_use]
    pub fn prime(&mut self) -> PRIME_W<PRIME_SPEC> {
        PRIME_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "usb1_ep_out prime register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prime::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIME_SPEC;
impl crate::RegisterSpec for PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prime::W`](W) writer structure"]
impl crate::Writable for PRIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prime to value 0"]
impl crate::Resettable for PRIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
