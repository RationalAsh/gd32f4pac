#[doc = "Register `HCH6STCTL` reader"]
pub type R = crate::R<Hch6stctlSpec>;
#[doc = "Register `HCH6STCTL` writer"]
pub type W = crate::W<Hch6stctlSpec>;
#[doc = "Field `PADDR` reader - Port address"]
pub type PaddrR = crate::FieldReader;
#[doc = "Field `PADDR` writer - Port address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HADDR` reader - HUB address"]
pub type HaddrR = crate::FieldReader;
#[doc = "Field `HADDR` writer - HUB address"]
pub type HaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ISOPCE` reader - Isochronous OUT payload continuation encoding"]
pub type IsopceR = crate::FieldReader;
#[doc = "Field `ISOPCE` writer - Isochronous OUT payload continuation encoding"]
pub type IsopceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSPLT` reader - Complete split enable"]
pub type CspltR = crate::BitReader;
#[doc = "Field `CSPLT` writer - Complete split enable"]
pub type CspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLEN` reader - Enable high speed split transaction"]
pub type SplenR = crate::BitReader;
#[doc = "Field `SPLEN` writer - Enable high speed split transaction"]
pub type SplenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    pub fn haddr(&self) -> HaddrR {
        HaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    pub fn isopce(&self) -> IsopceR {
        IsopceR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    pub fn csplt(&self) -> CspltR {
        CspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    pub fn splen(&self) -> SplenR {
        SplenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<Hch6stctlSpec> {
        PaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    pub fn haddr(&mut self) -> HaddrW<Hch6stctlSpec> {
        HaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    pub fn isopce(&mut self) -> IsopceW<Hch6stctlSpec> {
        IsopceW::new(self, 14)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    pub fn csplt(&mut self) -> CspltW<Hch6stctlSpec> {
        CspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    pub fn splen(&mut self) -> SplenW<Hch6stctlSpec> {
        SplenW::new(self, 31)
    }
}
#[doc = "host channel-6 split transaction control register (HCH6STCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch6stctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch6stctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch6stctlSpec;
impl crate::RegisterSpec for Hch6stctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch6stctl::R`](R) reader structure"]
impl crate::Readable for Hch6stctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hch6stctl::W`](W) writer structure"]
impl crate::Writable for Hch6stctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH6STCTL to value 0"]
impl crate::Resettable for Hch6stctlSpec {}
