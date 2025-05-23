#[doc = "Register `GOTGINTF` reader"]
pub type R = crate::R<GotgintfSpec>;
#[doc = "Register `GOTGINTF` writer"]
pub type W = crate::W<GotgintfSpec>;
#[doc = "Field `SESEND` reader - Session end"]
pub type SesendR = crate::BitReader;
#[doc = "Field `SESEND` writer - Session end"]
pub type SesendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPEND` reader - SRPEND"]
pub type SrpendR = crate::BitReader;
#[doc = "Field `SRPEND` writer - SRPEND"]
pub type SrpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPEND` reader - HNP end"]
pub type HnpendR = crate::BitReader;
#[doc = "Field `HNPEND` writer - HNP end"]
pub type HnpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPDET` reader - Host negotiation detected"]
pub type HnpdetR = crate::BitReader;
#[doc = "Field `HNPDET` writer - Host negotiation detected"]
pub type HnpdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTO` reader - A-device timeout"]
pub type AdtoR = crate::BitReader;
#[doc = "Field `ADTO` writer - A-device timeout"]
pub type AdtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DF` reader - Debounce finish"]
pub type DfR = crate::BitReader;
#[doc = "Field `DF` writer - Debounce finish"]
pub type DfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&self) -> SesendR {
        SesendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRPEND"]
    #[inline(always)]
    pub fn srpend(&self) -> SrpendR {
        SrpendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&self) -> HnpendR {
        HnpendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hnpdet(&self) -> HnpdetR {
        HnpdetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&self) -> AdtoR {
        AdtoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&mut self) -> SesendW<GotgintfSpec> {
        SesendW::new(self, 2)
    }
    #[doc = "Bit 8 - SRPEND"]
    #[inline(always)]
    pub fn srpend(&mut self) -> SrpendW<GotgintfSpec> {
        SrpendW::new(self, 8)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&mut self) -> HnpendW<GotgintfSpec> {
        HnpendW::new(self, 9)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hnpdet(&mut self) -> HnpdetW<GotgintfSpec> {
        HnpdetW::new(self, 17)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&mut self) -> AdtoW<GotgintfSpec> {
        AdtoW::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&mut self) -> DfW<GotgintfSpec> {
        DfW::new(self, 19)
    }
}
#[doc = "Global OTG interrupt register (GOTGINTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgintfSpec;
impl crate::RegisterSpec for GotgintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgintf::R`](R) reader structure"]
impl crate::Readable for GotgintfSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgintf::W`](W) writer structure"]
impl crate::Writable for GotgintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGINTF to value 0"]
impl crate::Resettable for GotgintfSpec {}
