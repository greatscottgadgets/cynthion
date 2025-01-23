#[doc = "Register `mode` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `mode` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `periodic` reader - periodic field"]
pub type PERIODIC_R = crate::BitReader;
#[doc = "Field `periodic` writer - periodic field"]
pub type PERIODIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - periodic field"]
    #[inline(always)]
    pub fn periodic(&self) -> PERIODIC_R {
        PERIODIC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - periodic field"]
    #[inline(always)]
    #[must_use]
    pub fn periodic(&mut self) -> PERIODIC_W<MODE_SPEC> {
        PERIODIC_W::new(self, 0)
    }
}
#[doc = "Timer mode. When ``periodic`` is set to 1 the counter will automatically be reset to the reload value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets mode to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u8 = 0;
}
