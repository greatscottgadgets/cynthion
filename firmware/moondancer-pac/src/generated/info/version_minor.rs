#[doc = "Register `version_minor` reader"]
pub type R = crate::R<VERSION_MINOR_SPEC>;
#[doc = "Field `version_minor` reader - Contains the Cynthion hardware minor revision number."]
pub type VERSION_MINOR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the Cynthion hardware minor revision number."]
    #[inline(always)]
    pub fn version_minor(&self) -> VERSION_MINOR_R {
        VERSION_MINOR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "info version_minor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_minor::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_MINOR_SPEC;
impl crate::RegisterSpec for VERSION_MINOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_minor::R`](R) reader structure"]
impl crate::Readable for VERSION_MINOR_SPEC {}
#[doc = "`reset()` method sets version_minor to value 0"]
impl crate::Resettable for VERSION_MINOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
