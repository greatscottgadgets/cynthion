#[doc = "Register `reset` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
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
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reset` writer - Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb0_ep_control reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reset to value 0"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
