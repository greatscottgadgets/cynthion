#[doc = "Register `Mode` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `Mode` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `pin_0` reader - pin_0 field"]
pub type PIN_0_R = crate::FieldReader;
#[doc = "Field `pin_0` writer - pin_0 field"]
pub type PIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&mut self) -> PIN_0_W<MODE_SPEC> {
        PIN_0_W::new(self, 0)
    }
}
#[doc = "Mode register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 2-bit wide and its possible values are defined by the :class:`PinMode` enumeration. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Mode to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u8 = 0;
}
