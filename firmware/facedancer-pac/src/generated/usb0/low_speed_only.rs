#[doc = "Register `low_speed_only` reader"]
pub type R = crate::R<LOW_SPEED_ONLY_SPEC>;
#[doc = "Register `low_speed_only` writer"]
pub type W = crate::W<LOW_SPEED_ONLY_SPEC>;
#[doc = "Field `low_speed_only` reader - Set this bit to '1' to force the device to operate at low speed."]
pub type LOW_SPEED_ONLY_R = crate::BitReader;
#[doc = "Field `low_speed_only` writer - Set this bit to '1' to force the device to operate at low speed."]
pub type LOW_SPEED_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at low speed."]
    #[inline(always)]
    pub fn low_speed_only(&self) -> LOW_SPEED_ONLY_R {
        LOW_SPEED_ONLY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at low speed."]
    #[inline(always)]
    #[must_use]
    pub fn low_speed_only(&mut self) -> LOW_SPEED_ONLY_W<LOW_SPEED_ONLY_SPEC> {
        LOW_SPEED_ONLY_W::new(self, 0)
    }
}
#[doc = "usb0 low_speed_only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_speed_only::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_speed_only::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_SPEED_ONLY_SPEC;
impl crate::RegisterSpec for LOW_SPEED_ONLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_speed_only::R`](R) reader structure"]
impl crate::Readable for LOW_SPEED_ONLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`low_speed_only::W`](W) writer structure"]
impl crate::Writable for LOW_SPEED_ONLY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets low_speed_only to value 0"]
impl crate::Resettable for LOW_SPEED_ONLY_SPEC {
    const RESET_VALUE: u32 = 0;
}
