#[doc = "Register `address` reader"]
pub type R = crate::R<ADDRESS_SPEC>;
#[doc = "Register `address` writer"]
pub type W = crate::W<ADDRESS_SPEC>;
#[doc = "Field `address` reader - Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `address` writer - Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<ADDRESS_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "usb0_ep_control address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRESS_SPEC;
impl crate::RegisterSpec for ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for ADDRESS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets address to value 0"]
impl crate::Resettable for ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
