#[doc = "Register `output` writer"]
pub type W = crate::W<OUTPUT_SPEC>;
#[doc = "Field `output` writer - "]
pub type OUTPUT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OUTPUT_W<OUTPUT_SPEC> {
        OUTPUT_W::new(self, 0)
    }
}
#[doc = "leds output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets output to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
