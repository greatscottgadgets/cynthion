#[doc = "Register `reload` reader"]
pub type R = crate::R<RELOAD_SPEC>;
#[doc = "Register `reload` writer"]
pub type W = crate::W<RELOAD_SPEC>;
#[doc = "Field `value` reader - value field"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `value` writer - value field"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - value field"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - value field"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<RELOAD_SPEC> {
        VALUE_W::new(self, 0)
    }
}
#[doc = "Reload value of counter. When counter reaches 0 is is automatically reloaded with this value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RELOAD_SPEC;
impl crate::RegisterSpec for RELOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reload::R`](R) reader structure"]
impl crate::Readable for RELOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reload::W`](W) writer structure"]
impl crate::Writable for RELOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reload to value 0"]
impl crate::Resettable for RELOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
