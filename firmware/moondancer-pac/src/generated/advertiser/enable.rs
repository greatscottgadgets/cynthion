#[doc = "Register `enable` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `enable` reader - Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<ENABLE_SPEC> {
        ENABLE_W::new(self, 0)
    }
}
#[doc = "advertiser enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
