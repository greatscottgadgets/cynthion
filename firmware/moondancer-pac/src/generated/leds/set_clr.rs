#[doc = "Register `SetClr` reader"]
pub type R = crate::R<SET_CLR_SPEC>;
#[doc = "Register `SetClr` writer"]
pub type W = crate::W<SET_CLR_SPEC>;
#[doc = "Field `pin_0_set` writer - pin_0_set field"]
pub type PIN_0_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_0_clr` writer - pin_0_clr field"]
pub type PIN_0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_1_set` writer - pin_1_set field"]
pub type PIN_1_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_1_clr` writer - pin_1_clr field"]
pub type PIN_1_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_2_set` writer - pin_2_set field"]
pub type PIN_2_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_2_clr` writer - pin_2_clr field"]
pub type PIN_2_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_3_set` writer - pin_3_set field"]
pub type PIN_3_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_3_clr` writer - pin_3_clr field"]
pub type PIN_3_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_4_set` writer - pin_4_set field"]
pub type PIN_4_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_4_clr` writer - pin_4_clr field"]
pub type PIN_4_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_5_set` writer - pin_5_set field"]
pub type PIN_5_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_5_clr` writer - pin_5_clr field"]
pub type PIN_5_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - pin_1_set field"]
    #[inline(always)]
    pub fn pin_1_set(&mut self) -> PIN_1_SET_W<SET_CLR_SPEC> {
        PIN_1_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - pin_1_clr field"]
    #[inline(always)]
    pub fn pin_1_clr(&mut self) -> PIN_1_CLR_W<SET_CLR_SPEC> {
        PIN_1_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - pin_2_set field"]
    #[inline(always)]
    pub fn pin_2_set(&mut self) -> PIN_2_SET_W<SET_CLR_SPEC> {
        PIN_2_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - pin_2_clr field"]
    #[inline(always)]
    pub fn pin_2_clr(&mut self) -> PIN_2_CLR_W<SET_CLR_SPEC> {
        PIN_2_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - pin_3_set field"]
    #[inline(always)]
    pub fn pin_3_set(&mut self) -> PIN_3_SET_W<SET_CLR_SPEC> {
        PIN_3_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - pin_3_clr field"]
    #[inline(always)]
    pub fn pin_3_clr(&mut self) -> PIN_3_CLR_W<SET_CLR_SPEC> {
        PIN_3_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - pin_4_set field"]
    #[inline(always)]
    pub fn pin_4_set(&mut self) -> PIN_4_SET_W<SET_CLR_SPEC> {
        PIN_4_SET_W::new(self, 8)
    }
    #[doc = "Bit 9 - pin_4_clr field"]
    #[inline(always)]
    pub fn pin_4_clr(&mut self) -> PIN_4_CLR_W<SET_CLR_SPEC> {
        PIN_4_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - pin_5_set field"]
    #[inline(always)]
    pub fn pin_5_set(&mut self) -> PIN_5_SET_W<SET_CLR_SPEC> {
        PIN_5_SET_W::new(self, 10)
    }
    #[doc = "Bit 11 - pin_5_clr field"]
    #[inline(always)]
    pub fn pin_5_clr(&mut self) -> PIN_5_CLR_W<SET_CLR_SPEC> {
        PIN_5_CLR_W::new(self, 11)
    }
}
#[doc = "Output set/clear register. This :class:`csr.Register` contains an array of ``pin_count`` write-only fields. Each field is 2-bit wide; writing it can modify its associated :class:`~Peripheral.Output` field as a side-effect. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"W\" }, \\]
- Writing `0b01` to a field sets its associated :class:`~Peripheral.Output` field. - Writing `0b10` to a field clears its associated :class:`~Peripheral.Output` field. - Writing `0b00` or `0b11` to a field has no side-effect. Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`set_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_CLR_SPEC;
impl crate::RegisterSpec for SET_CLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`set_clr::R`](R) reader structure"]
impl crate::Readable for SET_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set_clr::W`](W) writer structure"]
impl crate::Writable for SET_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SetClr to value 0"]
impl crate::Resettable for SET_CLR_SPEC {
    const RESET_VALUE: u16 = 0;
}
