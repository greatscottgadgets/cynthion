#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `connect` reader - connect field"]
pub type CONNECT_R = crate::BitReader;
#[doc = "Field `connect` writer - connect field"]
pub type CONNECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `low_speed_only` reader - low_speed_only field"]
pub type LOW_SPEED_ONLY_R = crate::BitReader;
#[doc = "Field `low_speed_only` writer - low_speed_only field"]
pub type LOW_SPEED_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `full_speed_only` reader - full_speed_only field"]
pub type FULL_SPEED_ONLY_R = crate::BitReader;
#[doc = "Field `full_speed_only` writer - full_speed_only field"]
pub type FULL_SPEED_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_1` reader - _1 field"]
pub type _1_R = crate::FieldReader;
#[doc = "Field `_1` writer - _1 field"]
pub type _1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - connect field"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - low_speed_only field"]
    #[inline(always)]
    pub fn low_speed_only(&self) -> LOW_SPEED_ONLY_R {
        LOW_SPEED_ONLY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - full_speed_only field"]
    #[inline(always)]
    pub fn full_speed_only(&self) -> FULL_SPEED_ONLY_R {
        FULL_SPEED_ONLY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - _1 field"]
    #[inline(always)]
    pub fn _1(&self) -> _1_R {
        _1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - connect field"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> CONNECT_W<CONTROL_SPEC> {
        CONNECT_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<CONTROL_SPEC> {
        _0_W::new(self, 1)
    }
    #[doc = "Bit 8 - low_speed_only field"]
    #[inline(always)]
    #[must_use]
    pub fn low_speed_only(&mut self) -> LOW_SPEED_ONLY_W<CONTROL_SPEC> {
        LOW_SPEED_ONLY_W::new(self, 8)
    }
    #[doc = "Bit 9 - full_speed_only field"]
    #[inline(always)]
    #[must_use]
    pub fn full_speed_only(&mut self) -> FULL_SPEED_ONLY_W<CONTROL_SPEC> {
        FULL_SPEED_ONLY_W::new(self, 9)
    }
    #[doc = "Bits 10:15 - _1 field"]
    #[inline(always)]
    #[must_use]
    pub fn _1(&mut self) -> _1_W<CONTROL_SPEC> {
        _1_W::new(self, 10)
    }
}
#[doc = "Control register connect: Set this bit to '1' to allow the associated USB device to connect to a host. low_speed_only: Set this bit to '1' to force the device to operate at low speed. full_speed_only: Set this bit to '1' to force the device to operate at full speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u16 = 0;
}
