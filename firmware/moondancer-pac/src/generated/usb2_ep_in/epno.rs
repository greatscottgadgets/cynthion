#[doc = "Register `epno` reader"]
pub type R = crate::R<EPNO_SPEC>;
#[doc = "Register `epno` writer"]
pub type W = crate::W<EPNO_SPEC>;
#[doc = "Field `epno` reader - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
pub type EPNO_R = crate::FieldReader;
#[doc = "Field `epno` writer - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
pub type EPNO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
    #[inline(always)]
    #[must_use]
    pub fn epno(&mut self) -> EPNO_W<EPNO_SPEC> {
        EPNO_W::new(self, 0)
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
#[doc = "usb2_ep_in epno register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epno::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epno::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPNO_SPEC;
impl crate::RegisterSpec for EPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epno::R`](R) reader structure"]
impl crate::Readable for EPNO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epno::W`](W) writer structure"]
impl crate::Writable for EPNO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets epno to value 0"]
impl crate::Resettable for EPNO_SPEC {
    const RESET_VALUE: u32 = 0;
}
