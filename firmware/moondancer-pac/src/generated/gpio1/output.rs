#[doc = "Register `Output` reader"]
pub type R = crate::R<OUTPUT_SPEC>;
#[doc = "Register `Output` writer"]
pub type W = crate::W<OUTPUT_SPEC>;
#[doc = "Field `pin_0` reader - pin_0 field"]
pub type PIN_0_R = crate::BitReader;
#[doc = "Field `pin_0` writer - pin_0 field"]
pub type PIN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_1` reader - pin_1 field"]
pub type PIN_1_R = crate::BitReader;
#[doc = "Field `pin_1` writer - pin_1 field"]
pub type PIN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_2` reader - pin_2 field"]
pub type PIN_2_R = crate::BitReader;
#[doc = "Field `pin_2` writer - pin_2 field"]
pub type PIN_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_3` reader - pin_3 field"]
pub type PIN_3_R = crate::BitReader;
#[doc = "Field `pin_3` writer - pin_3 field"]
pub type PIN_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_4` reader - pin_4 field"]
pub type PIN_4_R = crate::BitReader;
#[doc = "Field `pin_4` writer - pin_4 field"]
pub type PIN_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_5` reader - pin_5 field"]
pub type PIN_5_R = crate::BitReader;
#[doc = "Field `pin_5` writer - pin_5 field"]
pub type PIN_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_6` reader - pin_6 field"]
pub type PIN_6_R = crate::BitReader;
#[doc = "Field `pin_6` writer - pin_6 field"]
pub type PIN_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_7` reader - pin_7 field"]
pub type PIN_7_R = crate::BitReader;
#[doc = "Field `pin_7` writer - pin_7 field"]
pub type PIN_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - pin_0 field"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pin_1 field"]
    #[inline(always)]
    pub fn pin_1(&self) -> PIN_1_R {
        PIN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pin_2 field"]
    #[inline(always)]
    pub fn pin_2(&self) -> PIN_2_R {
        PIN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pin_3 field"]
    #[inline(always)]
    pub fn pin_3(&self) -> PIN_3_R {
        PIN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pin_4 field"]
    #[inline(always)]
    pub fn pin_4(&self) -> PIN_4_R {
        PIN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pin_5 field"]
    #[inline(always)]
    pub fn pin_5(&self) -> PIN_5_R {
        PIN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pin_6 field"]
    #[inline(always)]
    pub fn pin_6(&self) -> PIN_6_R {
        PIN_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pin_7 field"]
    #[inline(always)]
    pub fn pin_7(&self) -> PIN_7_R {
        PIN_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pin_0 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_0(&mut self) -> PIN_0_W<OUTPUT_SPEC> {
        PIN_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - pin_1 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_1(&mut self) -> PIN_1_W<OUTPUT_SPEC> {
        PIN_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - pin_2 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_2(&mut self) -> PIN_2_W<OUTPUT_SPEC> {
        PIN_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - pin_3 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_3(&mut self) -> PIN_3_W<OUTPUT_SPEC> {
        PIN_3_W::new(self, 3)
    }
    #[doc = "Bit 4 - pin_4 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_4(&mut self) -> PIN_4_W<OUTPUT_SPEC> {
        PIN_4_W::new(self, 4)
    }
    #[doc = "Bit 5 - pin_5 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_5(&mut self) -> PIN_5_W<OUTPUT_SPEC> {
        PIN_5_W::new(self, 5)
    }
    #[doc = "Bit 6 - pin_6 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_6(&mut self) -> PIN_6_W<OUTPUT_SPEC> {
        PIN_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - pin_7 field"]
    #[inline(always)]
    #[must_use]
    pub fn pin_7(&mut self) -> PIN_7_W<OUTPUT_SPEC> {
        PIN_7_W::new(self, 7)
    }
}
#[doc = "Output register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 1-bit wide and drives the output of its associated pin in the :attr:`Peripheral.pins` array, depending on its associated :class:`~Peripheral.Mode` field. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
