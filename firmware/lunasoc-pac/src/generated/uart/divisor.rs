#[doc = "Register `divisor` reader"]
pub struct R(crate::R<DIVISOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVISOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVISOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVISOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `divisor` writer"]
pub struct W(crate::W<DIVISOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVISOR_SPEC>;
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
impl From<crate::W<DIVISOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVISOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `divisor` reader - uart divisor register field"]
pub type DIVISOR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `divisor` writer - uart divisor register field"]
pub type DIVISOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVISOR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - uart divisor register field"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - uart divisor register field"]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DIVISOR_W<0> {
        DIVISOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart divisor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divisor](index.html) module"]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divisor::R](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divisor::W](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets divisor to value 0"]
impl crate::Resettable for DIVISOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
