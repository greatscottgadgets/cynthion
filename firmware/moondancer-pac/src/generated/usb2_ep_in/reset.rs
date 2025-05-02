#[doc = "Register `reset` reader"]
pub type R = crate::R<RESET_SPEC>;
#[doc = "Register `reset` writer"]
pub type W = crate::W<RESET_SPEC>;
#[doc = "Field `fifo` writer - fifo field"]
pub type FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_1` reader - _1 field"]
pub type _1_R = crate::FieldReader;
#[doc = "Field `_1` writer - _1 field"]
pub type _1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - _1 field"]
    #[inline(always)]
    pub fn _1(&self) -> _1_R {
        _1_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - fifo field"]
    #[inline(always)]
    pub fn fifo(&mut self) -> FIFO_W<RESET_SPEC> {
        FIFO_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - _1 field"]
    #[inline(always)]
    pub fn _1(&mut self) -> _1_W<RESET_SPEC> {
        _1_W::new(self, 1)
    }
}
#[doc = "Reset register fifo: A write to this field Clears the FIFO without transmitting.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets reset to value 0"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: u8 = 0;
}
