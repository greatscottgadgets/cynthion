#[doc = "Register `divisor` reader"]
pub type R = crate::R<DIVISOR_SPEC>;
#[doc = "Register `divisor` writer"]
pub type W = crate::W<DIVISOR_SPEC>;
#[doc = "Field `divisor` reader - uart1 divisor register field"]
pub type DIVISOR_R = crate::FieldReader<u16>;
#[doc = "Field `divisor` writer - uart1 divisor register field"]
pub type DIVISOR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - uart1 divisor register field"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - uart1 divisor register field"]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DIVISOR_W<DIVISOR_SPEC> {
        DIVISOR_W::new(self, 0)
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
#[doc = "uart1 divisor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divisor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divisor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divisor::R`](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divisor::W`](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets divisor to value 0"]
impl crate::Resettable for DIVISOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
