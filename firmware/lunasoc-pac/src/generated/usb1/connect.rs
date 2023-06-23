#[doc = "Register `connect` reader"]
pub struct R(crate::R<CONNECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONNECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONNECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONNECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `connect` writer"]
pub struct W(crate::W<CONNECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONNECT_SPEC>;
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
impl From<crate::W<CONNECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONNECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `connect` reader - Set this bit to '1' to allow the associated USB device to connect to a host."]
pub type CONNECT_R = crate::BitReader<bool>;
#[doc = "Field `connect` writer - Set this bit to '1' to allow the associated USB device to connect to a host."]
pub type CONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONNECT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to allow the associated USB device to connect to a host."]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to allow the associated USB device to connect to a host."]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> CONNECT_W<0> {
        CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb1 connect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [connect](index.html) module"]
pub struct CONNECT_SPEC;
impl crate::RegisterSpec for CONNECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [connect::R](R) reader structure"]
impl crate::Readable for CONNECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [connect::W](W) writer structure"]
impl crate::Writable for CONNECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets connect to value 0"]
impl crate::Resettable for CONNECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
