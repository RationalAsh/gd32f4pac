#[doc = "Register `SNTCFG1` reader"]
pub type R = crate::R<Sntcfg1Spec>;
#[doc = "Register `SNTCFG1` writer"]
pub type W = crate::W<Sntcfg1Spec>;
#[doc = "Field `ASET` reader - Address setup time"]
pub type AsetR = crate::FieldReader;
#[doc = "Field `ASET` writer - Address setup time"]
pub type AsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHLD` reader - Address hold time"]
pub type AhldR = crate::FieldReader;
#[doc = "Field `AHLD` writer - Address hold time"]
pub type AhldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSET` reader - Data setup time"]
pub type DsetR = crate::FieldReader;
#[doc = "Field `DSET` writer - Data setup time"]
pub type DsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BuslatR = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BuslatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CKDIV` reader - Synchronous clock divide ratio"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Synchronous clock divide ratio"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLAT` reader - Data latency for NOR Flash"]
pub type DlatR = crate::FieldReader;
#[doc = "Field `DLAT` writer - Data latency for NOR Flash"]
pub type DlatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASYNCMOD` reader - Asynchronous access mode"]
pub type AsyncmodR = crate::FieldReader;
#[doc = "Field `ASYNCMOD` writer - Asynchronous access mode"]
pub type AsyncmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> AsetR {
        AsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AhldR {
        AhldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DsetR {
        DsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BuslatR {
        BuslatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&self) -> DlatR {
        DlatR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&self) -> AsyncmodR {
        AsyncmodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&mut self) -> AsetW<Sntcfg1Spec> {
        AsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&mut self) -> AhldW<Sntcfg1Spec> {
        AhldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&mut self) -> DsetW<Sntcfg1Spec> {
        DsetW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&mut self) -> BuslatW<Sntcfg1Spec> {
        BuslatW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<Sntcfg1Spec> {
        CkdivW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&mut self) -> DlatW<Sntcfg1Spec> {
        DlatW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&mut self) -> AsyncmodW<Sntcfg1Spec> {
        AsyncmodW::new(self, 28)
    }
}
#[doc = "SRAM/NOR flash timing configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sntcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sntcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sntcfg1Spec;
impl crate::RegisterSpec for Sntcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sntcfg1::R`](R) reader structure"]
impl crate::Readable for Sntcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sntcfg1::W`](W) writer structure"]
impl crate::Writable for Sntcfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNTCFG1 to value 0x0fff_ffff"]
impl crate::Resettable for Sntcfg1Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
