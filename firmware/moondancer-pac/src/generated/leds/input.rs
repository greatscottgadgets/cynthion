#[doc = "Register `Input` reader"]
pub type R = crate::R<INPUT_SPEC>;
#[doc = "Register `Input` writer"]
pub type W = crate::W<INPUT_SPEC>;
#[doc = "Field `pin_0` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_0_R = crate::BitReader;
#[doc = "Field `pin_1` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_1_R = crate::BitReader;
#[doc = "Field `pin_2` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_2_R = crate::BitReader;
#[doc = "Field `pin_3` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_3_R = crate::BitReader;
#[doc = "Field `pin_4` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_4_R = crate::BitReader;
#[doc = "Field `pin_5` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type PIN_5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_0(&self) -> PIN_0_R {
        PIN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_1(&self) -> PIN_1_R {
        PIN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_2(&self) -> PIN_2_R {
        PIN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_3(&self) -> PIN_3_R {
        PIN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_4(&self) -> PIN_4_R {
        PIN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn pin_5(&self) -> PIN_5_R {
        PIN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Input to value 0"]
impl crate::Resettable for INPUT_SPEC {
    const RESET_VALUE: u8 = 0;
}
