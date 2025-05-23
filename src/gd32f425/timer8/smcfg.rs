#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SmcfgSpec>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SmcfgSpec>;
#[doc = "Field `SMC` reader - Slave mode control"]
pub type SmcR = crate::FieldReader;
#[doc = "Field `SMC` writer - Slave mode control"]
pub type SmcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TrgsR = crate::FieldReader;
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TrgsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master-slave mode"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - Master-slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TrgsR {
        TrgsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&mut self) -> SmcW<SmcfgSpec> {
        SmcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&mut self) -> TrgsW<SmcfgSpec> {
        TrgsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<SmcfgSpec> {
        MsmW::new(self, 7)
    }
}
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcfgSpec;
impl crate::RegisterSpec for SmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SmcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SmcfgSpec {}
