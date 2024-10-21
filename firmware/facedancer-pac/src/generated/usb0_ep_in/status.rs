#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `nak` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type NAK_R = crate::FieldReader<u16>;
#[doc = "Field `epno` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type EPNO_R = crate::FieldReader;
#[doc = "Field `_0` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `idle` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `have` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type HAVE_R = crate::BitReader;
#[doc = "Field `pid` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PID_R = crate::BitReader;
#[doc = "Field `_1` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type _1_R = crate::FieldReader;
#[doc = "Field `_1` writer - TODO amaranth_soc/csr/reg.py:471"]
pub type _1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn _1(&self) -> _1_R {
        _1_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<STATUS_SPEC> {
        _0_W::new(self, 20)
    }
    #[doc = "Bits 27:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    #[must_use]
    pub fn _1(&mut self) -> _1_W<STATUS_SPEC> {
        _1_W::new(self, 27)
    }
}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
