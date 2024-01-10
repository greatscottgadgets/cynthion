#[doc = "Register `moder` reader"]
pub type R = crate::R<MODER_SPEC>;
#[doc = "Register `moder` writer"]
pub type W = crate::W<MODER_SPEC>;
#[doc = "Field `moder` reader - gpioa moder register field"]
pub type MODER_R = crate::FieldReader;
#[doc = "Field `moder` writer - gpioa moder register field"]
pub type MODER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gpioa moder register field"]
    #[inline(always)]
    pub fn moder(&self) -> MODER_R {
        MODER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpioa moder register field"]
    #[inline(always)]
    #[must_use]
    pub fn moder(&mut self) -> MODER_W<MODER_SPEC> {
        MODER_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "gpioa moder register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets moder to value 0"]
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: u32 = 0;
}
