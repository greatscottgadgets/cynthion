#[doc = "Register `pid` reader"]
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pid` writer"]
pub struct W(crate::W<PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_SPEC>;
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
impl From<crate::W<PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pid` reader - Contains the current PID toggle bit for the given endpoint."]
pub type PID_R = crate::BitReader<bool>;
#[doc = "Field `pid` writer - Contains the current PID toggle bit for the given endpoint."]
pub type PID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PID_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb2_ep_out pid register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](index.html) module"]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid::R](R) reader structure"]
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pid::W](W) writer structure"]
impl crate::Writable for PID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pid to value 0"]
impl crate::Resettable for PID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
