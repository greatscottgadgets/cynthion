#[doc = "Register `connect` reader"]
pub type R = crate::R<CONNECT_SPEC>;
#[doc = "Register `connect` writer"]
pub type W = crate::W<CONNECT_SPEC>;
#[doc = "Field `connect` reader - Set this bit to '1' to allow the associated USB device to connect to a host."]
pub type CONNECT_R = crate::BitReader;
#[doc = "Field `connect` writer - Set this bit to '1' to allow the associated USB device to connect to a host."]
pub type CONNECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to '1' to allow the associated USB device to connect to a host."]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to '1' to allow the associated USB device to connect to a host."]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> CONNECT_W<CONNECT_SPEC> {
        CONNECT_W::new(self, 0)
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
#[doc = "usb2 connect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`connect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`connect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONNECT_SPEC;
impl crate::RegisterSpec for CONNECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`connect::R`](R) reader structure"]
impl crate::Readable for CONNECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`connect::W`](W) writer structure"]
impl crate::Writable for CONNECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets connect to value 0"]
impl crate::Resettable for CONNECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
