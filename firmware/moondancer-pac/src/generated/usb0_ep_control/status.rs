#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `address` reader - address field"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `epno` reader - epno field"]
pub type EPNO_R = crate::FieldReader;
#[doc = "Field `have` reader - have field"]
pub type HAVE_R = crate::BitReader;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - address field"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - epno field"]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - have field"]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 13:15 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<STATUS_SPEC> {
        _0_W::new(self, 13)
    }
}
#[doc = "Status register address: Holds the current device's active USB address. epno: The endpoint number associated with the most recently captured SETUP packet. have: `1` iff data is available in the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u16 = 0;
}
