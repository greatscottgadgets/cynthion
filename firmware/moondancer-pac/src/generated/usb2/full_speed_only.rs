#[doc = "Register `full_speed_only` reader"]
pub type R = crate::R<FULL_SPEED_ONLY_SPEC>;
#[doc = "Register `full_speed_only` writer"]
pub type W = crate::W<FULL_SPEED_ONLY_SPEC>;
#[doc = "Field `full_speed_only` reader - Set this bit to '1' to force the device to operate at full speed."]
pub type FULL_SPEED_ONLY_R = crate::BitReader;
#[doc = "Field `full_speed_only` writer - Set this bit to '1' to force the device to operate at full speed."]
pub type FULL_SPEED_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at full speed."]
    #[inline(always)]
    pub fn full_speed_only(&self) -> FULL_SPEED_ONLY_R {
        FULL_SPEED_ONLY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to force the device to operate at full speed."]
    #[inline(always)]
    #[must_use]
    pub fn full_speed_only(&mut self) -> FULL_SPEED_ONLY_W<FULL_SPEED_ONLY_SPEC> {
        FULL_SPEED_ONLY_W::new(self, 0)
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
#[doc = "usb2 full_speed_only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`full_speed_only::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`full_speed_only::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FULL_SPEED_ONLY_SPEC;
impl crate::RegisterSpec for FULL_SPEED_ONLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`full_speed_only::R`](R) reader structure"]
impl crate::Readable for FULL_SPEED_ONLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`full_speed_only::W`](W) writer structure"]
impl crate::Writable for FULL_SPEED_ONLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets full_speed_only to value 0"]
impl crate::Resettable for FULL_SPEED_ONLY_SPEC {
    const RESET_VALUE: u32 = 0;
}
