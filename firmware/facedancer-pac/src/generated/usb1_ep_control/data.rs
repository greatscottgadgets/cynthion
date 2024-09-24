#[doc = "Register `data` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Field `data` reader - A FIFO that returns the bytes from the most recently captured SETUP packet. Reading a byte from this register advances the FIFO. The first eight bytes read from this contain the core SETUP packet."]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - A FIFO that returns the bytes from the most recently captured SETUP packet. Reading a byte from this register advances the FIFO. The first eight bytes read from this contain the core SETUP packet."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "usb1_ep_control data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`reset()` method sets data to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
