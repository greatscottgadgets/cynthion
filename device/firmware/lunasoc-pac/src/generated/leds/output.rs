#[doc = "Register `output` writer"]
pub struct W(crate::W<OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_SPEC>;
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
impl From<crate::W<OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `output` writer - leds output register field"]
pub type OUTPUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTPUT_SPEC, u8, u8, 6, O>;
impl W {
    #[doc = "Bits 0:5 - leds output register field"]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OUTPUT_W<0> {
        OUTPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "leds output register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](index.html) module"]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [output::W](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets output to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
