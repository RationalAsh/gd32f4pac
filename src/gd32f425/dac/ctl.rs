#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type Den0R = crate::BitReader;
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type Den0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type Dboff0R = crate::BitReader;
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type Dboff0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type Dten0R = crate::BitReader;
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type Dten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type Dtsel0R = crate::FieldReader;
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type Dtsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWM0` reader - DAC0 noise wave mode"]
pub type Dwm0R = crate::FieldReader;
#[doc = "Field `DWM0` writer - DAC0 noise wave mode"]
pub type Dwm0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DWBW0` reader - DAC0 noise wave bit width"]
pub type Dwbw0R = crate::FieldReader;
#[doc = "Field `DWBW0` writer - DAC0 noise wave bit width"]
pub type Dwbw0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type Ddmaen0R = crate::BitReader;
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type Ddmaen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDUDRIE0` reader - DAC0 DMA Underrun Interrupt enable"]
pub type Ddudrie0R = crate::BitReader;
#[doc = "Field `DDUDRIE0` writer - DAC0 DMA Underrun Interrupt enable"]
pub type Ddudrie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub type Den1R = crate::BitReader;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub type Den1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub type Dboff1R = crate::BitReader;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub type Dboff1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub type Dten1R = crate::BitReader;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub type Dten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub type Dtsel1R = crate::FieldReader;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub type Dtsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWM1` reader - DAC1 noise wave mode"]
pub type Dwm1R = crate::FieldReader;
#[doc = "Field `DWM1` writer - DAC1 noise wave mode"]
pub type Dwm1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DWBW1` reader - DAC1 noise wave bit width"]
pub type Dwbw1R = crate::FieldReader;
#[doc = "Field `DWBW1` writer - DAC1 noise wave bit width"]
pub type Dwbw1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub type Ddmaen1R = crate::BitReader;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub type Ddmaen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDUDRIE1` reader - DAC1 DMA Underrun Interrupt enable"]
pub type Ddudrie1R = crate::BitReader;
#[doc = "Field `DDUDRIE1` writer - DAC1 DMA Underrun Interrupt enable"]
pub type Ddudrie1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> Den0R {
        Den0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> Dboff0R {
        Dboff0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> Dten0R {
        Dten0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> Dtsel0R {
        Dtsel0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&self) -> Dwm0R {
        Dwm0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&self) -> Dwbw0R {
        Dwbw0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> Ddmaen0R {
        Ddmaen0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&self) -> Ddudrie0R {
        Ddudrie0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> Den1R {
        Den1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> Dboff1R {
        Dboff1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> Dten1R {
        Dten1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> Dtsel1R {
        Dtsel1R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&self) -> Dwm1R {
        Dwm1R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&self) -> Dwbw1R {
        Dwbw1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> Ddmaen1R {
        Ddmaen1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&self) -> Ddudrie1R {
        Ddudrie1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&mut self) -> Den0W<CtlSpec> {
        Den0W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&mut self) -> Dboff0W<CtlSpec> {
        Dboff0W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&mut self) -> Dten0W<CtlSpec> {
        Dten0W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&mut self) -> Dtsel0W<CtlSpec> {
        Dtsel0W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&mut self) -> Dwm0W<CtlSpec> {
        Dwm0W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&mut self) -> Dwbw0W<CtlSpec> {
        Dwbw0W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&mut self) -> Ddmaen0W<CtlSpec> {
        Ddmaen0W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&mut self) -> Ddudrie0W<CtlSpec> {
        Ddudrie0W::new(self, 13)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&mut self) -> Den1W<CtlSpec> {
        Den1W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&mut self) -> Dboff1W<CtlSpec> {
        Dboff1W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&mut self) -> Dten1W<CtlSpec> {
        Dten1W::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&mut self) -> Dtsel1W<CtlSpec> {
        Dtsel1W::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&mut self) -> Dwm1W<CtlSpec> {
        Dwm1W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&mut self) -> Dwbw1W<CtlSpec> {
        Dwbw1W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&mut self) -> Ddmaen1W<CtlSpec> {
        Ddmaen1W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&mut self) -> Ddudrie1W<CtlSpec> {
        Ddudrie1W::new(self, 29)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
