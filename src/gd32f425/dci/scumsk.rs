#[doc = "Register `SCUMSK` reader"]
pub type R = crate::R<ScumskSpec>;
#[doc = "Register `SCUMSK` writer"]
pub type W = crate::W<ScumskSpec>;
#[doc = "Field `FSM` reader - Frame Start Code unMask Bits in Embedded Synchronous Mode"]
pub type FsmR = crate::FieldReader;
#[doc = "Field `FSM` writer - Frame Start Code unMask Bits in Embedded Synchronous Mode"]
pub type FsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSM` reader - Line Start Code unMask Bits in Embedded Synchronous Mode"]
pub type LsmR = crate::FieldReader;
#[doc = "Field `LSM` writer - Line Start Code unMask Bits in Embedded Synchronous Mode"]
pub type LsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEM` reader - Line End Code unMask Bits in Embedded Synchronous Mode"]
pub type LemR = crate::FieldReader;
#[doc = "Field `LEM` writer - Line End Code unMask Bits in Embedded Synchronous Mode"]
pub type LemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEM` reader - Frame End Code unMask Bits in Embedded Synchronous Mode"]
pub type FemR = crate::FieldReader;
#[doc = "Field `FEM` writer - Frame End Code unMask Bits in Embedded Synchronous Mode"]
pub type FemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Start Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fsm(&self) -> FsmR {
        FsmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line Start Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LsmR {
        LsmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line End Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn lem(&self) -> LemR {
        LemR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame End Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fem(&self) -> FemR {
        FemR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Start Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fsm(&mut self) -> FsmW<ScumskSpec> {
        FsmW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line Start Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn lsm(&mut self) -> LsmW<ScumskSpec> {
        LsmW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line End Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn lem(&mut self) -> LemW<ScumskSpec> {
        LemW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame End Code unMask Bits in Embedded Synchronous Mode"]
    #[inline(always)]
    pub fn fem(&mut self) -> FemW<ScumskSpec> {
        FemW::new(self, 24)
    }
}
#[doc = "Synchronization codes unmask register\n\nYou can [`read`](crate::Reg::read) this register and get [`scumsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scumsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScumskSpec;
impl crate::RegisterSpec for ScumskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scumsk::R`](R) reader structure"]
impl crate::Readable for ScumskSpec {}
#[doc = "`write(|w| ..)` method takes [`scumsk::W`](W) writer structure"]
impl crate::Writable for ScumskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUMSK to value 0"]
impl crate::Resettable for ScumskSpec {}
