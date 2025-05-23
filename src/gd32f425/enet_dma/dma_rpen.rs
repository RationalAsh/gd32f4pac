#[doc = "Register `DMA_RPEN` reader"]
pub type R = crate::R<DmaRpenSpec>;
#[doc = "Register `DMA_RPEN` writer"]
pub type W = crate::W<DmaRpenSpec>;
#[doc = "Field `RPE` reader - Receive poll enable"]
pub type RpeR = crate::FieldReader<u32>;
#[doc = "Field `RPE` writer - Receive poll enable"]
pub type RpeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll enable"]
    #[inline(always)]
    pub fn rpe(&self) -> RpeR {
        RpeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll enable"]
    #[inline(always)]
    pub fn rpe(&mut self) -> RpeW<DmaRpenSpec> {
        RpeW::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive poll enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRpenSpec;
impl crate::RegisterSpec for DmaRpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rpen::R`](R) reader structure"]
impl crate::Readable for DmaRpenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rpen::W`](W) writer structure"]
impl crate::Writable for DmaRpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RPEN to value 0"]
impl crate::Resettable for DmaRpenSpec {}
