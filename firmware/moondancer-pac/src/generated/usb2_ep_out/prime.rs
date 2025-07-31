#[doc = "Register `prime` reader"]
pub type R = crate::R<PRIME_SPEC>;
#[doc = "Register `prime` writer"]
pub type W = crate::W<PRIME_SPEC>;
#[doc = "Field `primed` writer - primed field"]
pub type PRIMED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - primed field"]
    #[inline(always)]
    pub fn primed(&mut self) -> PRIMED_W<PRIME_SPEC> {
        PRIMED_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0_W<PRIME_SPEC> {
        _0_W::new(self, 1)
    }
}
#[doc = "Prime register primed: Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Note that this does not apply to the control endpoint. Once the control endpoint has received a packet it will be un-primed and need to be re-primed before it can receive again. This is to ensure that we can establish an order on the receipt of the setup packet and any associated data. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets.\n\nYou can [`read`](crate::Reg::read) this register and get [`prime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIME_SPEC;
impl crate::RegisterSpec for PRIME_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prime::R`](R) reader structure"]
impl crate::Readable for PRIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prime::W`](W) writer structure"]
impl crate::Writable for PRIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets prime to value 0"]
impl crate::Resettable for PRIME_SPEC {
    const RESET_VALUE: u8 = 0;
}
