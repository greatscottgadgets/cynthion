#[doc = "Register `low_speed_only` reader"]
pub struct R(crate::R<LOW_SPEED_ONLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_SPEED_ONLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_SPEED_ONLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_SPEED_ONLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `low_speed_only` writer"]
pub struct W(crate::W<LOW_SPEED_ONLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOW_SPEED_ONLY_SPEC>;
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
impl From<crate::W<LOW_SPEED_ONLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOW_SPEED_ONLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `low_speed_only` reader - Set this bit to '1' to force the device to operate at low speed."]
pub type LOW_SPEED_ONLY_R = crate::BitReader<bool>;
#[doc = "Field `low_speed_only` writer - Set this bit to '1' to force the device to operate at low speed."]
pub type LOW_SPEED_ONLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOW_SPEED_ONLY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at low speed."]
    #[inline(always)]
    pub fn low_speed_only(&self) -> LOW_SPEED_ONLY_R {
        LOW_SPEED_ONLY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at low speed."]
    #[inline(always)]
    #[must_use]
    pub fn low_speed_only(&mut self) -> LOW_SPEED_ONLY_W<0> {
        LOW_SPEED_ONLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb1 low_speed_only register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_speed_only](index.html) module"]
pub struct LOW_SPEED_ONLY_SPEC;
impl crate::RegisterSpec for LOW_SPEED_ONLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_speed_only::R](R) reader structure"]
impl crate::Readable for LOW_SPEED_ONLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [low_speed_only::W](W) writer structure"]
impl crate::Writable for LOW_SPEED_ONLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets low_speed_only to value 0"]
impl crate::Resettable for LOW_SPEED_ONLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
