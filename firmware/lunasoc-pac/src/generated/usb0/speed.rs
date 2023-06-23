#[doc = "Register `speed` reader"]
pub struct R(crate::R<SPEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `speed` reader - Indicates the current speed of the USB device. 0 indicates High; 1 => Full, 2 => Low, and 3 => SuperSpeed (incl SuperSpeed+)."]
pub type SPEED_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Indicates the current speed of the USB device. 0 indicates High; 1 => Full, 2 => Low, and 3 => SuperSpeed (incl SuperSpeed+)."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 3) as u8)
    }
}
#[doc = "usb0 speed register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [speed](index.html) module"]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [speed::R](R) reader structure"]
impl crate::Readable for SPEED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets speed to value 0"]
impl crate::Resettable for SPEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
