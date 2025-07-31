#[doc = "Register `Output` reader"]
pub type R = crate::R<OUTPUT_SPEC>;
#[doc = "Register `Output` writer"]
pub type W = crate::W<OUTPUT_SPEC>;
#[doc = "Field `pin_0` reader - pin_0 field"]
pub type PIN_0_R = crate::BitReader;
#[doc = "Field `pin_0` writer - pin_0 field"]
pub type PIN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&mut self) -> PIN_0_W<OUTPUT_SPEC> {
        PIN_0_W::new(self, 0)
    }
}
#[doc = "Output register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 1-bit wide and drives the output of its associated pin in the :attr:`Peripheral.pins` array, depending on its associated :class:`~Peripheral.Mode` field. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`output::R`](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Output to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: u8 = 0;
}
