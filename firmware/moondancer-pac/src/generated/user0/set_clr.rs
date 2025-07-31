#[doc = "Register `SetClr` reader"]
pub type R = crate::R<SET_CLR_SPEC>;
#[doc = "Register `SetClr` writer"]
pub type W = crate::W<SET_CLR_SPEC>;
#[doc = "Field `pin_0_set` writer - pin_0_set field"]
pub type PIN_0_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_0_clr` writer - pin_0_clr field"]
pub type PIN_0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - pin_0_set field"]
    #[inline(always)]
    pub fn pin_0_set(&mut self) -> PIN_0_SET_W<SET_CLR_SPEC> {
        PIN_0_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - pin_0_clr field"]
    #[inline(always)]
    pub fn pin_0_clr(&mut self) -> PIN_0_CLR_W<SET_CLR_SPEC> {
        PIN_0_CLR_W::new(self, 1)
    }
}
#[doc = "Output set/clear register. This :class:`csr.Register` contains an array of ``pin_count`` write-only fields. Each field is 2-bit wide; writing it can modify its associated :class:`~Peripheral.Output` field as a side-effect. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"W\" }, \\]
- Writing `0b01` to a field sets its associated :class:`~Peripheral.Output` field. - Writing `0b10` to a field clears its associated :class:`~Peripheral.Output` field. - Writing `0b00` or `0b11` to a field has no side-effect. Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`set_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_CLR_SPEC;
impl crate::RegisterSpec for SET_CLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`set_clr::R`](R) reader structure"]
impl crate::Readable for SET_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set_clr::W`](W) writer structure"]
impl crate::Writable for SET_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SetClr to value 0"]
impl crate::Resettable for SET_CLR_SPEC {
    const RESET_VALUE: u8 = 0;
}
