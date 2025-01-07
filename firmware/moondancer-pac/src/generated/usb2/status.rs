#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `speed` reader - speed field"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - speed field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bits 2:7 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<STATUS_SPEC> {
        _0_W::new(self, 2)
    }
}
#[doc = "Status register speed: Indicates the current speed of the USB device. 0 indicates High; 1 => Full, 2 => Low, and 3 => SuperSpeed (incl SuperSpeed+).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u8 = 0;
}
