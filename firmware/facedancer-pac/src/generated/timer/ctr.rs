#[doc = "Register `ctr` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `ctr` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `ctr` reader - timer ctr register field"]
pub type CTR_R = crate::FieldReader<u32>;
#[doc = "Field `ctr` writer - timer ctr register field"]
pub type CTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - timer ctr register field"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer ctr register field"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<CTR_SPEC> {
        CTR_W::new(self, 0)
    }
}
#[doc = "timer ctr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctr to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
