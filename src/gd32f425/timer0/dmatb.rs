#[doc = "Register `DMATB` reader"]
pub type R = crate::R<DmatbSpec>;
#[doc = "Register `DMATB` writer"]
pub type W = crate::W<DmatbSpec>;
#[doc = "Field `DMATB` reader - DMA transfer buffer"]
pub type DmatbR = crate::FieldReader<u16>;
#[doc = "Field `DMATB` writer - DMA transfer buffer"]
pub type DmatbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&self) -> DmatbR {
        DmatbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&mut self) -> DmatbW<DmatbSpec> {
        DmatbW::new(self, 0)
    }
}
#[doc = "DMA transfer buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatbSpec;
impl crate::RegisterSpec for DmatbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatb::R`](R) reader structure"]
impl crate::Readable for DmatbSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatb::W`](W) writer structure"]
impl crate::Writable for DmatbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DmatbSpec {}
