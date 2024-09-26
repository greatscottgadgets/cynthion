#[doc = "Register `rx` reader"]
pub type R = crate::R<RX_SPEC>;
#[doc = "Register `rx` writer"]
pub type W = crate::W<RX_SPEC>;
#[doc = "Field `data` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `ready` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:31 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 32 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 32) & 1) != 0)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_SPEC;
impl crate::RegisterSpec for RX_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rx::R`](R) reader structure"]
impl crate::Readable for RX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx::W`](W) writer structure"]
impl crate::Writable for RX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets rx to value 0"]
impl crate::Resettable for RX_SPEC {
    const RESET_VALUE: u64 = 0;
}
