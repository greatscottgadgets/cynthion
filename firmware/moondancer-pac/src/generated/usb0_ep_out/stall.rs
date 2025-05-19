#[doc = "Register `stall` reader"]
pub type R = crate::R<STALL_SPEC>;
#[doc = "Register `stall` writer"]
pub type W = crate::W<STALL_SPEC>;
#[doc = "Field `stalled` writer - stalled field"]
pub type STALLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0` reader - _0 field"]
pub type _0_R = crate::FieldReader;
#[doc = "Field `_0` writer - _0 field"]
pub type _0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&self) -> _0_R {
        _0_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - stalled field"]
    #[inline(always)]
    #[must_use]
    pub fn stalled(&mut self) -> STALLED_W<STALL_SPEC> {
        STALLED_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    #[must_use]
    pub fn _0(&mut self) -> _0_W<STALL_SPEC> {
        _0_W::new(self, 1)
    }
}
#[doc = "Stall register stalled: Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STALL_SPEC;
impl crate::RegisterSpec for STALL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stall::R`](R) reader structure"]
impl crate::Readable for STALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stall::W`](W) writer structure"]
impl crate::Writable for STALL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets stall to value 0"]
impl crate::Resettable for STALL_SPEC {
    const RESET_VALUE: u8 = 0;
}
