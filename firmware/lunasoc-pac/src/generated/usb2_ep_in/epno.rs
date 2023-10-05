#[doc = "Register `epno` reader"]
pub struct R(crate::R<EPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `epno` writer"]
pub struct W(crate::W<EPNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPNO_SPEC>;
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
impl From<crate::W<EPNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `epno` reader - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
pub type EPNO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `epno` writer - Contains the endpoint the enqueued packet is to be transmitted on. Writing this register marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
pub type EPNO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPNO_SPEC, u8, u8, 4, O>;
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
    pub fn epno(&mut self) -> EPNO_W<0> {
        EPNO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb2_ep_in epno register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epno](index.html) module"]
pub struct EPNO_SPEC;
impl crate::RegisterSpec for EPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epno::R](R) reader structure"]
impl crate::Readable for EPNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epno::W](W) writer structure"]
impl crate::Writable for EPNO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets epno to value 0"]
impl crate::Resettable for EPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
