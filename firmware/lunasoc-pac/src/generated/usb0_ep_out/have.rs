#[doc = "Register `have` reader"]
pub struct R(crate::R<HAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `have` reader - `1` iff data is available in the FIFO."]
pub type HAVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - `1` iff data is available in the FIFO."]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "usb0_ep_out have register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [have](index.html) module"]
pub struct HAVE_SPEC;
impl crate::RegisterSpec for HAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [have::R](R) reader structure"]
impl crate::Readable for HAVE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets have to value 0"]
impl crate::Resettable for HAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
