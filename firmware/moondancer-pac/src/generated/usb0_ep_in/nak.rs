#[doc = "Register `nak` reader"]
pub type R = crate::R<NAK_SPEC>;
#[doc = "Field `nak` reader - Read-only register. Contains a bitmask of endpoints that have responded with a NAK since the last read of this register."]
pub type NAK_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Read-only register. Contains a bitmask of endpoints that have responded with a NAK since the last read of this register."]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "usb0_ep_in nak register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nak::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAK_SPEC;
impl crate::RegisterSpec for NAK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nak::R`](R) reader structure"]
impl crate::Readable for NAK_SPEC {}
#[doc = "`reset()` method sets nak to value 0"]
impl crate::Resettable for NAK_SPEC {
    const RESET_VALUE: u32 = 0;
}
