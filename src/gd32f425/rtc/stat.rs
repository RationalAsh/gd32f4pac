#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `ALRM0WF` reader - Alarm 0 configuration can be write flag"]
pub type Alrm0wfR = crate::BitReader;
#[doc = "Field `ALRM1WF` reader - Alarm 1 configuration can be write flag"]
pub type Alrm1wfR = crate::BitReader;
#[doc = "Field `WTWF` reader - Wakeup timer write enable flag"]
pub type WtwfR = crate::BitReader;
#[doc = "Field `SOPF` reader - Shift function operation pending flag"]
pub type SopfR = crate::BitReader;
#[doc = "Field `SOPF` writer - Shift function operation pending flag"]
pub type SopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YCM` reader - Year configuration mark"]
pub type YcmR = crate::BitReader;
#[doc = "Field `RSYNF` reader - Register synchronization flag"]
pub type RsynfR = crate::BitReader;
#[doc = "Field `RSYNF` writer - Register synchronization flag"]
pub type RsynfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization state flag"]
pub type InitfR = crate::BitReader;
#[doc = "Field `INITM` reader - Enter initialization mode"]
pub type InitmR = crate::BitReader;
#[doc = "Field `INITM` writer - Enter initialization mode"]
pub type InitmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0F` reader - Alarm-0 occurs flag"]
pub type Alrm0fR = crate::BitReader;
#[doc = "Field `ALRM0F` writer - Alarm-0 occurs flag"]
pub type Alrm0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM1F` reader - Alarm-1 occurs flag"]
pub type Alrm1fR = crate::BitReader;
#[doc = "Field `ALRM1F` writer - Alarm-1 occurs flag"]
pub type Alrm1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTF` reader - Wakeup timer flag"]
pub type WtfR = crate::BitReader;
#[doc = "Field `WTF` writer - Wakeup timer flag"]
pub type WtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOVRF` reader - Time-stamp overflow flag"]
pub type TsovrfR = crate::BitReader;
#[doc = "Field `TSOVRF` writer - Time-stamp overflow flag"]
pub type TsovrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP0F` reader - RTC_TAMP0 detected flag"]
pub type Tp0fR = crate::BitReader;
#[doc = "Field `TP0F` writer - RTC_TAMP0 detected flag"]
pub type Tp0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1F` reader - RTC_TAMP1 detected flag"]
pub type Tp1fR = crate::BitReader;
#[doc = "Field `TP1F` writer - RTC_TAMP1 detected flag"]
pub type Tp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPF` reader - Smooth calibration pending flag"]
pub type ScpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm 0 configuration can be write flag"]
    #[inline(always)]
    pub fn alrm0wf(&self) -> Alrm0wfR {
        Alrm0wfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm 1 configuration can be write flag"]
    #[inline(always)]
    pub fn alrm1wf(&self) -> Alrm1wfR {
        Alrm1wfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write enable flag"]
    #[inline(always)]
    pub fn wtwf(&self) -> WtwfR {
        WtwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift function operation pending flag"]
    #[inline(always)]
    pub fn sopf(&self) -> SopfR {
        SopfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Year configuration mark"]
    #[inline(always)]
    pub fn ycm(&self) -> YcmR {
        YcmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Register synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RsynfR {
        RsynfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization state flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enter initialization mode"]
    #[inline(always)]
    pub fn initm(&self) -> InitmR {
        InitmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm-0 occurs flag"]
    #[inline(always)]
    pub fn alrm0f(&self) -> Alrm0fR {
        Alrm0fR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm-1 occurs flag"]
    #[inline(always)]
    pub fn alrm1f(&self) -> Alrm1fR {
        Alrm1fR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wtf(&self) -> WtfR {
        WtfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&self) -> TsovrfR {
        TsovrfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC_TAMP0 detected flag"]
    #[inline(always)]
    pub fn tp0f(&self) -> Tp0fR {
        Tp0fR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detected flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> Tp1fR {
        Tp1fR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Smooth calibration pending flag"]
    #[inline(always)]
    pub fn scpf(&self) -> ScpfR {
        ScpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift function operation pending flag"]
    #[inline(always)]
    pub fn sopf(&mut self) -> SopfW<StatSpec> {
        SopfW::new(self, 3)
    }
    #[doc = "Bit 5 - Register synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&mut self) -> RsynfW<StatSpec> {
        RsynfW::new(self, 5)
    }
    #[doc = "Bit 7 - Enter initialization mode"]
    #[inline(always)]
    pub fn initm(&mut self) -> InitmW<StatSpec> {
        InitmW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm-0 occurs flag"]
    #[inline(always)]
    pub fn alrm0f(&mut self) -> Alrm0fW<StatSpec> {
        Alrm0fW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm-1 occurs flag"]
    #[inline(always)]
    pub fn alrm1f(&mut self) -> Alrm1fW<StatSpec> {
        Alrm1fW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wtf(&mut self) -> WtfW<StatSpec> {
        WtfW::new(self, 10)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TsfW<StatSpec> {
        TsfW::new(self, 11)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&mut self) -> TsovrfW<StatSpec> {
        TsovrfW::new(self, 12)
    }
    #[doc = "Bit 13 - RTC_TAMP0 detected flag"]
    #[inline(always)]
    pub fn tp0f(&mut self) -> Tp0fW<StatSpec> {
        Tp0fW::new(self, 13)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detected flag"]
    #[inline(always)]
    pub fn tp1f(&mut self) -> Tp1fW<StatSpec> {
        Tp1fW::new(self, 14)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0x07"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x07;
}
