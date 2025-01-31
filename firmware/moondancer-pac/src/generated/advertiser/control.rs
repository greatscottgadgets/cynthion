#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `enable` writer - enable field"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - enable field"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONTROL_SPEC> {
        ENABLE_W::new(self, 0)
    }
}
#[doc = "Control register enable : Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u8 = 0;
}
