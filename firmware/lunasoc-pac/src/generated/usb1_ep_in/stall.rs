#[doc = "Register `stall` reader"]
pub struct R(crate::R<STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `stall` writer"]
pub struct W(crate::W<STALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STALL_SPEC>;
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
impl From<crate::W<STALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `stall` reader - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `stall` writer - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STALL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<0> {
        STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb1_ep_in stall register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall](index.html) module"]
pub struct STALL_SPEC;
impl crate::RegisterSpec for STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall::R](R) reader structure"]
impl crate::Readable for STALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stall::W](W) writer structure"]
impl crate::Writable for STALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stall to value 0"]
impl crate::Resettable for STALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
