#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `epno` reader - epno field"]
pub type EPNO_R = crate::FieldReader;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `have` reader - have field"]
pub type HAVE_R = crate::BitReader;
#[doc = "Field `pid` reader - pid field"]
pub type PID_R = crate::BitReader;
#[doc = "Field `_1` reader - _1 field"]
pub type _1_R = crate::FieldReader;
#[doc = "Field `_1` writer - _1 field"]
pub type _1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - epno field"]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - have field"]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pid field"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - _1 field"]
    #[inline(always)]
    pub fn _1(&self) -> _1_R {
        _1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<STATUS_SPEC> {
        _0_W::new(self, 4)
    }
    #[doc = "Bits 10:15 - _1 field"]
    #[inline(always)]
    #[must_use]
    pub fn _1(&mut self) -> _1_W<STATUS_SPEC> {
        _1_W::new(self, 10)
    }
}
#[doc = "Status register epno: Contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received. have: `1` iff data is available in the FIFO. pid: Contains the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
