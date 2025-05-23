#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Field `FS` reader - Frame Start Code in Embedded Synchronous Mode"]
pub type FsR = crate::FieldReader;
#[doc = "Field `FS` writer - Frame Start Code in Embedded Synchronous Mode"]
pub type FsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LS` reader - Line Start Code in Embedded Synchronous Mode"]
pub type LsR = crate::FieldReader;
#[doc = "Field `LS` writer - Line Start Code in Embedded Synchronous Mode"]
pub type LsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LE` reader - Line End Code in Embedded Synchronous Mode"]
pub type LeR = crate::FieldReader;
#[doc = "Field `LE` writer - Line End Code in Embedded Synchronous Mode"]
pub type LeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FE` reader - Frame End Code in Embedded Synchronous Mode"]
pub type FeR = crate::FieldReader;
#[doc = "Field `FE` writer - Frame End Code in Embedded Synchronous Mode"]
pub type FeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Start Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line Start Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line End Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame End Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Start Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<ScSpec> {
        FsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line Start Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<ScSpec> {
        LsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line End Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<ScSpec> {
        LeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame End Code in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<ScSpec> {
        FeW::new(self, 24)
    }
}
#[doc = "Synchronization codes register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {}
