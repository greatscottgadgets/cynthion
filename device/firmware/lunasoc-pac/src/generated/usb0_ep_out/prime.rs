#[doc = "Register `prime` writer"]
pub struct W(crate::W<PRIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `prime` writer - Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
pub type PRIME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIME_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
    #[inline(always)]
    #[must_use]
    pub fn prime(&mut self) -> PRIME_W<0> {
        PRIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb0_ep_out prime register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prime](index.html) module"]
pub struct PRIME_SPEC;
impl crate::RegisterSpec for PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prime::W](W) writer structure"]
impl crate::Writable for PRIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prime to value 0"]
impl crate::Resettable for PRIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
