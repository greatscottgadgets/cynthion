#[doc = "Register `tx_data` writer"]
pub struct W(crate::W<TX_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TX_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_data` writer - uart tx_data register field"]
pub type TX_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_DATA_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - uart tx_data register field"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<0> {
        TX_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart tx_data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data](index.html) module"]
pub struct TX_DATA_SPEC;
impl crate::RegisterSpec for TX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_data::W](W) writer structure"]
impl crate::Writable for TX_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_data to value 0"]
impl crate::Resettable for TX_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
