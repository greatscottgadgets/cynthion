#[doc = "Register `data` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Register `data` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `byte` writer - byte field"]
pub type BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - byte field"]
    #[inline(always)]
    pub fn byte(&mut self) -> BYTE_W<DATA_SPEC> {
        BYTE_W::new(self, 0)
    }
}
#[doc = "Data register Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
