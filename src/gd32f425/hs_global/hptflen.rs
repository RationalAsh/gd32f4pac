#[doc = "Register `HPTFLEN` reader"]
pub type R = crate::R<HptflenSpec>;
#[doc = "Register `HPTFLEN` writer"]
pub type W = crate::W<HptflenSpec>;
#[doc = "Field `HPTXFSAR` reader - Host periodic TxFIFO start address"]
pub type HptxfsarR = crate::FieldReader<u16>;
#[doc = "Field `HPTXFSAR` writer - Host periodic TxFIFO start address"]
pub type HptxfsarW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HPTXFD` reader - Host periodic TxFIFO depth"]
pub type HptxfdR = crate::FieldReader<u16>;
#[doc = "Field `HPTXFD` writer - Host periodic TxFIFO depth"]
pub type HptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&self) -> HptxfsarR {
        HptxfsarR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&self) -> HptxfdR {
        HptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&mut self) -> HptxfsarW<HptflenSpec> {
        HptxfsarW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&mut self) -> HptxfdW<HptflenSpec> {
        HptxfdW::new(self, 16)
    }
}
#[doc = "Host periodic transmit FIFO size register (HPTFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`hptflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptflenSpec;
impl crate::RegisterSpec for HptflenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptflen::R`](R) reader structure"]
impl crate::Readable for HptflenSpec {}
#[doc = "`write(|w| ..)` method takes [`hptflen::W`](W) writer structure"]
impl crate::Writable for HptflenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPTFLEN to value 0x0200_0600"]
impl crate::Resettable for HptflenSpec {
    const RESET_VALUE: u32 = 0x0200_0600;
}
