#[doc = "Register `SAMCS` reader"]
pub type R = crate::R<SamcsSpec>;
#[doc = "Register `SAMCS` writer"]
pub type W = crate::W<SamcsSpec>;
#[doc = "Field `SAMEN` reader - SAM_V interface enable"]
pub type SamenR = crate::BitReader;
#[doc = "Field `SAMEN` writer - SAM_V interface enable"]
pub type SamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOEN` reader - SAM_V interface timeout detect enable"]
pub type StoenR = crate::BitReader;
#[doc = "Field `STOEN` writer - SAM_V interface timeout detect enable"]
pub type StoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFFIE` reader - Txframe fall interrupt enable"]
pub type TffieR = crate::BitReader;
#[doc = "Field `TFFIE` writer - Txframe fall interrupt enable"]
pub type TffieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFRIE` reader - Txframe rise interrupt enable"]
pub type TfrieR = crate::BitReader;
#[doc = "Field `TFRIE` writer - Txframe rise interrupt enable"]
pub type TfrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFFIE` reader - Rxframe fall interrupt enable"]
pub type RffieR = crate::BitReader;
#[doc = "Field `RFFIE` writer - Rxframe fall interrupt enable"]
pub type RffieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFRIE` reader - Rxframe rise interrupt enable"]
pub type RfrieR = crate::BitReader;
#[doc = "Field `RFRIE` writer - Rxframe rise interrupt enable"]
pub type RfrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXF` reader - Level of Txframe signal"]
pub type TxfR = crate::BitReader;
#[doc = "Field `RXF` reader - Level of Rxframe signal"]
pub type RxfR = crate::BitReader;
#[doc = "Field `TFF` reader - Txframe fall flag"]
pub type TffR = crate::BitReader;
#[doc = "Field `TFF` writer - Txframe fall flag"]
pub type TffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFR` reader - Txframe rise flag"]
pub type TfrR = crate::BitReader;
#[doc = "Field `TFR` writer - Txframe rise flag"]
pub type TfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFF` reader - Rxframe fall flag"]
pub type RffR = crate::BitReader;
#[doc = "Field `RFF` writer - Rxframe fall flag"]
pub type RffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFR` reader - Rxframe rise flag"]
pub type RfrR = crate::BitReader;
#[doc = "Field `RFR` writer - Rxframe rise flag"]
pub type RfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SamenR {
        SamenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> StoenR {
        StoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TffieR {
        TffieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TfrieR {
        TfrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RffieR {
        RffieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RfrieR {
        RfrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of Txframe signal"]
    #[inline(always)]
    pub fn txf(&self) -> TxfR {
        TxfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of Rxframe signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RfrR {
        RfrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&mut self) -> SamenW<SamcsSpec> {
        SamenW::new(self, 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&mut self) -> StoenW<SamcsSpec> {
        StoenW::new(self, 1)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&mut self) -> TffieW<SamcsSpec> {
        TffieW::new(self, 4)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&mut self) -> TfrieW<SamcsSpec> {
        TfrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RffieW<SamcsSpec> {
        RffieW::new(self, 6)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&mut self) -> RfrieW<SamcsSpec> {
        RfrieW::new(self, 7)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TffW<SamcsSpec> {
        TffW::new(self, 12)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&mut self) -> TfrW<SamcsSpec> {
        TfrW::new(self, 13)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&mut self) -> RffW<SamcsSpec> {
        RffW::new(self, 14)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&mut self) -> RfrW<SamcsSpec> {
        RfrW::new(self, 15)
    }
}
#[doc = "SAM control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`samcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SamcsSpec;
impl crate::RegisterSpec for SamcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samcs::R`](R) reader structure"]
impl crate::Readable for SamcsSpec {}
#[doc = "`write(|w| ..)` method takes [`samcs::W`](W) writer structure"]
impl crate::Writable for SamcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMCS to value 0"]
impl crate::Resettable for SamcsSpec {}
