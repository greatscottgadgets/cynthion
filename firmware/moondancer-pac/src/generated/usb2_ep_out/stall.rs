#[doc = "Register `stall` reader"]
pub type R = crate::R<STALL_SPEC>;
#[doc = "Register `stall` writer"]
pub type W = crate::W<STALL_SPEC>;
#[doc = "Field `stall` reader - Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `stall` writer - Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<STALL_SPEC> {
        STALL_W::new(self, 0)
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
#[doc = "usb2_ep_out stall register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STALL_SPEC;
impl crate::RegisterSpec for STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall::R`](R) reader structure"]
impl crate::Readable for STALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stall::W`](W) writer structure"]
impl crate::Writable for STALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stall to value 0"]
impl crate::Resettable for STALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
