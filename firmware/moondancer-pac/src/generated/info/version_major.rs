#[doc = "Register `version_major` reader"]
pub type R = crate::R<VERSION_MAJOR_SPEC>;
#[doc = "Field `version_major` reader - Contains the Cynthion hardware major revision number."]
pub type VERSION_MAJOR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the Cynthion hardware major revision number."]
    #[inline(always)]
    pub fn version_major(&self) -> VERSION_MAJOR_R {
        VERSION_MAJOR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "info version_major register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_major::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_MAJOR_SPEC;
impl crate::RegisterSpec for VERSION_MAJOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_major::R`](R) reader structure"]
impl crate::Readable for VERSION_MAJOR_SPEC {}
#[doc = "`reset()` method sets version_major to value 0"]
impl crate::Resettable for VERSION_MAJOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
