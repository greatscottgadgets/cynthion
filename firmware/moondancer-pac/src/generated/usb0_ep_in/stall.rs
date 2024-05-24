#[doc = "Register `stall` reader"]
pub type R = crate::R<STALL_SPEC>;
#[doc = "Register `stall` writer"]
pub type W = crate::W<STALL_SPEC>;
#[doc = "Field `stall` reader - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `stall` writer - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this register contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this register will automatically be cleared when a new SETUP token is received."]
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
#[doc = "usb0_ep_in stall register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
