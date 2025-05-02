#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `nak` reader - nak field"]
pub type NAK_R = crate::FieldReader<u16>;
#[doc = "Field `epno` reader - epno field"]
pub type EPNO_R = crate::FieldReader;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `idle` reader - idle field"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `have` reader - have field"]
pub type HAVE_R = crate::BitReader;
#[doc = "Field `pid` reader - pid field"]
pub type PID_R = crate::BitReader;
#[doc = "Field `_1` reader - _1 field"]
pub type _1_R = crate::FieldReader;
#[doc = "Field `_1` writer - _1 field"]
pub type _1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - nak field"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - epno field"]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - idle field"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - have field"]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - pid field"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - _1 field"]
    #[inline(always)]
    pub fn _1(&self) -> _1_R {
        _1_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - _0 field"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0_W<STATUS_SPEC> {
        _0_W::new(self, 20)
    }
    #[doc = "Bits 27:31 - _1 field"]
    #[inline(always)]
    pub fn _1(&mut self) -> _1_W<STATUS_SPEC> {
        _1_W::new(self, 27)
    }
}
#[doc = "Status register nak: Contains a bitmask of endpoints that have responded with a NAK since the last read of this register. epno: Contains the endpoint being transmitted on. idle: This value is `1` if no packet is actively being transmitted. have: This value is `1` if data is present in the transmit FIFO. pid: Contains the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
