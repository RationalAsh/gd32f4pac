#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UpieR = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieR = crate::BitReader;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1IE` reader - Channel 1 capture/compare interrupt enable"]
pub type Ch1ieR = crate::BitReader;
#[doc = "Field `CH1IE` writer - Channel 1 capture/compare interrupt enable"]
pub type Ch1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2IE` reader - Channel 2 capture/compare interrupt enable"]
pub type Ch2ieR = crate::BitReader;
#[doc = "Field `CH2IE` writer - Channel 2 capture/compare interrupt enable"]
pub type Ch2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3IE` reader - Channel 3 capture/compare interrupt enable"]
pub type Ch3ieR = crate::BitReader;
#[doc = "Field `CH3IE` writer - Channel 3 capture/compare interrupt enable"]
pub type Ch3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTIE` reader - commutation interrupt enable"]
pub type CmtieR = crate::BitReader;
#[doc = "Field `CMTIE` writer - commutation interrupt enable"]
pub type CmtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TrgieR = crate::BitReader;
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TrgieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub type BrkieR = crate::BitReader;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub type BrkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UpdenR = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UpdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type Ch0denR = crate::BitReader;
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type Ch0denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DEN` reader - Channel 1 capture/compare DMA request enable"]
pub type Ch1denR = crate::BitReader;
#[doc = "Field `CH1DEN` writer - Channel 1 capture/compare DMA request enable"]
pub type Ch1denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2DEN` reader - Channel 2 capture/compare DMA request enable"]
pub type Ch2denR = crate::BitReader;
#[doc = "Field `CH2DEN` writer - Channel 2 capture/compare DMA request enable"]
pub type Ch2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3DEN` reader - Channel 3 capture/compare DMA request enable"]
pub type Ch3denR = crate::BitReader;
#[doc = "Field `CH3DEN` writer - Channel 3 capture/compare DMA request enable"]
pub type Ch3denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTDEN` reader - Commutation DMA request enable"]
pub type CmtdenR = crate::BitReader;
#[doc = "Field `CMTDEN` writer - Commutation DMA request enable"]
pub type CmtdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TrgdenR = crate::BitReader;
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TrgdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> Ch0ieR {
        Ch0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> Ch1ieR {
        Ch1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> Ch2ieR {
        Ch2ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> Ch3ieR {
        Ch3ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - commutation interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CmtieR {
        CmtieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TrgieR {
        TrgieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BrkieR {
        BrkieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UpdenR {
        UpdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> Ch0denR {
        Ch0denR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> Ch1denR {
        Ch1denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> Ch2denR {
        Ch2denR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> Ch3denR {
        Ch3denR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    pub fn cmtden(&self) -> CmtdenR {
        CmtdenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TrgdenR {
        TrgdenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<DmaintenSpec> {
        UpieW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> Ch0ieW<DmaintenSpec> {
        Ch0ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&mut self) -> Ch1ieW<DmaintenSpec> {
        Ch1ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&mut self) -> Ch2ieW<DmaintenSpec> {
        Ch2ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&mut self) -> Ch3ieW<DmaintenSpec> {
        Ch3ieW::new(self, 4)
    }
    #[doc = "Bit 5 - commutation interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&mut self) -> CmtieW<DmaintenSpec> {
        CmtieW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&mut self) -> TrgieW<DmaintenSpec> {
        TrgieW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&mut self) -> BrkieW<DmaintenSpec> {
        BrkieW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UpdenW<DmaintenSpec> {
        UpdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> Ch0denW<DmaintenSpec> {
        Ch0denW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&mut self) -> Ch1denW<DmaintenSpec> {
        Ch1denW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&mut self) -> Ch2denW<DmaintenSpec> {
        Ch2denW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&mut self) -> Ch3denW<DmaintenSpec> {
        Ch3denW::new(self, 12)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    pub fn cmtden(&mut self) -> CmtdenW<DmaintenSpec> {
        CmtdenW::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&mut self) -> TrgdenW<DmaintenSpec> {
        TrgdenW::new(self, 14)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {}
