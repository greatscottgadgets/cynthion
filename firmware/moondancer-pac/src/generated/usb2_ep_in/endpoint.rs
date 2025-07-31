#[doc = "Register `endpoint` reader"]
pub type R = crate::R<ENDPOINT_SPEC>;
#[doc = "Register `endpoint` writer"]
pub type W = crate::W<ENDPOINT_SPEC>;
#[doc = "Field `number` writer - number field"]
pub type NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - number field"]
    #[inline(always)]
    pub fn number(&mut self) -> NUMBER_W<ENDPOINT_SPEC> {
        NUMBER_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0_W<ENDPOINT_SPEC> {
        _0_W::new(self, 4)
    }
}
#[doc = "Endpoint register number: Contains the endpoint the enqueued packet is to be transmitted on. Writing to this field marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd.\n\nYou can [`read`](crate::Reg::read) this register and get [`endpoint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpoint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPOINT_SPEC;
impl crate::RegisterSpec for ENDPOINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`endpoint::R`](R) reader structure"]
impl crate::Readable for ENDPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endpoint::W`](W) writer structure"]
impl crate::Writable for ENDPOINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets endpoint to value 0"]
impl crate::Resettable for ENDPOINT_SPEC {
    const RESET_VALUE: u8 = 0;
}
