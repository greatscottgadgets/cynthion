#[doc = "Register `idr` reader"]
pub type R = crate::R<IDR_SPEC>;
#[doc = "Field `idr` reader - gpioa idr register field"]
pub type IDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - gpioa idr register field"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "gpioa idr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDR_SPEC {}
#[doc = "`reset()` method sets idr to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
