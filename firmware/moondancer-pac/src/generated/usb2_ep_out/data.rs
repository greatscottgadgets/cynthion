#[doc = "Register `data` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Register `data` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `byte` reader - byte field"]
pub type BYTE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - byte field"]
    #[inline(always)]
    pub fn byte(&self) -> BYTE_R {
        BYTE_R::new(self.bits)
    }
}
impl W {}
#[doc = "Data register Read-only register. A FIFO that returns the bytes from the most recently captured OUT transaction. Reading a byte from this register advances the FIFO. byte: Contains the most recently received byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets data to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u8 = 0;
}
