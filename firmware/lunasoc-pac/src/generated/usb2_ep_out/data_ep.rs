#[doc = "Register `data_ep` reader"]
pub struct R(crate::R<DATA_EP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_EP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_EP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_EP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `data_ep` reader - Register that contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received."]
pub type DATA_EP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Register that contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received."]
    #[inline(always)]
    pub fn data_ep(&self) -> DATA_EP_R {
        DATA_EP_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "usb2_ep_out data_ep register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_ep](index.html) module"]
pub struct DATA_EP_SPEC;
impl crate::RegisterSpec for DATA_EP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_ep::R](R) reader structure"]
impl crate::Readable for DATA_EP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_ep to value 0"]
impl crate::Resettable for DATA_EP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
