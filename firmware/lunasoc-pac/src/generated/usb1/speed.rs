#[doc = "Register `speed` reader"]
pub type R = crate::R<SPEED_SPEC>;
#[doc = "Field `speed` reader - Indicates the current speed of the USB device. 0 indicates High; 1 => Full, 2 => Low, and 3 => SuperSpeed (incl SuperSpeed+)."]
pub type SPEED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Indicates the current speed of the USB device. 0 indicates High; 1 => Full, 2 => Low, and 3 => SuperSpeed (incl SuperSpeed+)."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 3) as u8)
    }
}
#[doc = "usb1 speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`speed::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`speed::R`](R) reader structure"]
impl crate::Readable for SPEED_SPEC {}
#[doc = "`reset()` method sets speed to value 0"]
impl crate::Resettable for SPEED_SPEC {
    const RESET_VALUE: u32 = 0;
}
