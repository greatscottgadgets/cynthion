#[doc = "Register `enable` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `enabled` writer - enabled field"]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - enabled field"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<ENABLE_SPEC> {
        ENABLED_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _0 field"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0_W<ENABLE_SPEC> {
        _0_W::new(self, 1)
    }
}
#[doc = "Enable register enabled: Controls whether any data can be received on any primed OUT endpoint. This bit is automatically cleared on receive in order to give the controller time to read data from the FIFO. It must be re-enabled once the FIFO has been emptied.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u8 = 0;
}
