#[doc = "Register `ALRM1TD` reader"]
pub type R = crate::R<Alrm1tdSpec>;
#[doc = "Register `ALRM1TD` writer"]
pub type W = crate::W<Alrm1tdSpec>;
#[doc = "Field `SCU` reader - Second units in BCD code."]
pub type ScuR = crate::FieldReader;
#[doc = "Field `SCU` writer - Second units in BCD code."]
pub type ScuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCT` reader - Second tens in BCD code."]
pub type SctR = crate::FieldReader;
#[doc = "Field `SCT` writer - Second tens in BCD code."]
pub type SctW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSKS` reader - Alarm seconds mask bit"]
pub type MsksR = crate::BitReader;
#[doc = "Field `MSKS` writer - Alarm seconds mask bit"]
pub type MsksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNU` reader - Minute units in BCD code."]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD code."]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD code."]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD code."]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSKM` reader - Alarm minutes mask bit"]
pub type MskmR = crate::BitReader;
#[doc = "Field `MSKM` writer - Alarm minutes mask bit"]
pub type MskmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRU` reader - Hour units in BCD code."]
pub type HruR = crate::FieldReader;
#[doc = "Field `HRU` writer - Hour units in BCD code."]
pub type HruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HRT` reader - Hour tens in BCD code."]
pub type HrtR = crate::FieldReader;
#[doc = "Field `HRT` writer - Hour tens in BCD code."]
pub type HrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM flag"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM flag"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSKH` reader - Alarm hours mask bit"]
pub type MskhR = crate::BitReader;
#[doc = "Field `MSKH` writer - Alarm hours mask bit"]
pub type MskhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYU` reader - Date units or week day in BCD code."]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - Date units or week day in BCD code."]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Date tens in BCD code."]
pub type DaytR = crate::FieldReader;
#[doc = "Field `DAYT` writer - Date tens in BCD code."]
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOWS` reader - Day of the week selected"]
pub type DowsR = crate::BitReader;
#[doc = "Field `DOWS` writer - Day of the week selected"]
pub type DowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSKD` reader - Alarm date mask bit"]
pub type MskdR = crate::BitReader;
#[doc = "Field `MSKD` writer - Alarm date mask bit"]
pub type MskdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD code."]
    #[inline(always)]
    pub fn scu(&self) -> ScuR {
        ScuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code."]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask bit"]
    #[inline(always)]
    pub fn msks(&self) -> MsksR {
        MsksR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code."]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code."]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask bit"]
    #[inline(always)]
    pub fn mskm(&self) -> MskmR {
        MskmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD code."]
    #[inline(always)]
    pub fn hru(&self) -> HruR {
        HruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code."]
    #[inline(always)]
    pub fn hrt(&self) -> HrtR {
        HrtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM flag"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm hours mask bit"]
    #[inline(always)]
    pub fn mskh(&self) -> MskhR {
        MskhR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or week day in BCD code."]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD code."]
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Day of the week selected"]
    #[inline(always)]
    pub fn dows(&self) -> DowsR {
        DowsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm date mask bit"]
    #[inline(always)]
    pub fn mskd(&self) -> MskdR {
        MskdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD code."]
    #[inline(always)]
    pub fn scu(&mut self) -> ScuW<Alrm1tdSpec> {
        ScuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code."]
    #[inline(always)]
    pub fn sct(&mut self) -> SctW<Alrm1tdSpec> {
        SctW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm seconds mask bit"]
    #[inline(always)]
    pub fn msks(&mut self) -> MsksW<Alrm1tdSpec> {
        MsksW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code."]
    #[inline(always)]
    pub fn mnu(&mut self) -> MnuW<Alrm1tdSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code."]
    #[inline(always)]
    pub fn mnt(&mut self) -> MntW<Alrm1tdSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm minutes mask bit"]
    #[inline(always)]
    pub fn mskm(&mut self) -> MskmW<Alrm1tdSpec> {
        MskmW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD code."]
    #[inline(always)]
    pub fn hru(&mut self) -> HruW<Alrm1tdSpec> {
        HruW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code."]
    #[inline(always)]
    pub fn hrt(&mut self) -> HrtW<Alrm1tdSpec> {
        HrtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM flag"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<Alrm1tdSpec> {
        PmW::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm hours mask bit"]
    #[inline(always)]
    pub fn mskh(&mut self) -> MskhW<Alrm1tdSpec> {
        MskhW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or week day in BCD code."]
    #[inline(always)]
    pub fn dayu(&mut self) -> DayuW<Alrm1tdSpec> {
        DayuW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD code."]
    #[inline(always)]
    pub fn dayt(&mut self) -> DaytW<Alrm1tdSpec> {
        DaytW::new(self, 28)
    }
    #[doc = "Bit 30 - Day of the week selected"]
    #[inline(always)]
    pub fn dows(&mut self) -> DowsW<Alrm1tdSpec> {
        DowsW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm date mask bit"]
    #[inline(always)]
    pub fn mskd(&mut self) -> MskdW<Alrm1tdSpec> {
        MskdW::new(self, 31)
    }
}
#[doc = "Alarm 1 time and date register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrm1td::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrm1td::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alrm1tdSpec;
impl crate::RegisterSpec for Alrm1tdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrm1td::R`](R) reader structure"]
impl crate::Readable for Alrm1tdSpec {}
#[doc = "`write(|w| ..)` method takes [`alrm1td::W`](W) writer structure"]
impl crate::Writable for Alrm1tdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRM1TD to value 0"]
impl crate::Resettable for Alrm1tdSpec {}
