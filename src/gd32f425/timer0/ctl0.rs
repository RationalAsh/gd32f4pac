#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDIS` reader - Update disable"]
pub type UpdisR = crate::BitReader;
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPS` reader - Update source"]
pub type UpsR = crate::BitReader;
#[doc = "Field `UPS` writer - Update source"]
pub type UpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SpmR = crate::BitReader;
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM` reader - Counter aligns mode selection"]
pub type CamR = crate::FieldReader;
#[doc = "Field `CAM` writer - Counter aligns mode selection"]
pub type CamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub type ArseR = crate::BitReader;
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub type ArseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UpdisR {
        UpdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UpsR {
        UpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SpmR {
        SpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CamR {
        CamR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ArseR {
        ArseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<Ctl0Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&mut self) -> UpdisW<Ctl0Spec> {
        UpdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&mut self) -> UpsW<Ctl0Spec> {
        UpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&mut self) -> SpmW<Ctl0Spec> {
        SpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<Ctl0Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&mut self) -> CamW<Ctl0Spec> {
        CamW::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&mut self) -> ArseW<Ctl0Spec> {
        ArseW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<Ctl0Spec> {
        CkdivW::new(self, 8)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
