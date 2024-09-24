#[doc = "Register `data` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `data` writer - Write-only register. Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write-only register. Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "usb2_ep_in data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
