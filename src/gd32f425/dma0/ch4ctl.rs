#[doc = "Register `CH4CTL` reader"]
pub type R = crate::R<Ch4ctlSpec>;
#[doc = "Register `CH4CTL` writer"]
pub type W = crate::W<Ch4ctlSpec>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIE` reader - Enable bit for single data mode exception interrupt"]
pub type SdeieR = crate::BitReader;
#[doc = "Field `SDEIE` writer - Enable bit for single data mode exception interrupt"]
pub type SdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIE` reader - Enable bit for tranfer access error interrupt"]
pub type TaeieR = crate::BitReader;
#[doc = "Field `TAEIE` writer - Enable bit for tranfer access error interrupt"]
pub type TaeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIE` reader - Enable bit for half transfer finish interrupt"]
pub type HtfieR = crate::BitReader;
#[doc = "Field `HTFIE` writer - Enable bit for half transfer finish interrupt"]
pub type HtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIE` reader - Enable bit for full transfer finish interrupt"]
pub type FtfieR = crate::BitReader;
#[doc = "Field `FTFIE` writer - Enable bit for full transfer finish interrupt"]
pub type FtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCS` reader - Transfer flow controller select"]
pub type TfcsR = crate::BitReader;
#[doc = "Field `TFCS` writer - Transfer flow controller select"]
pub type TfcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - Transfer mode"]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Transfer mode"]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMEN` reader - Circulation mode enable"]
pub type CmenR = crate::BitReader;
#[doc = "Field `CMEN` writer - Circulation mode enable"]
pub type CmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PnagaR = crate::BitReader;
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub type MnagaR = crate::BitReader;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub type MnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Transfer width of peripheral"]
pub type PwidthR = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Transfer width of peripheral"]
pub type PwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Transfer width of memory"]
pub type MwidthR = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Transfer width of memory"]
pub type MwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAIF` reader - Peripheral address increment fixed"]
pub type PaifR = crate::BitReader;
#[doc = "Field `PAIF` writer - Peripheral address increment fixed"]
pub type PaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - Priority level"]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Priority level"]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SBMEN` reader - Switch-buffer mode enable"]
pub type SbmenR = crate::BitReader;
#[doc = "Field `SBMEN` writer - Switch-buffer mode enable"]
pub type SbmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBS` reader - Memory buffer select"]
pub type MbsR = crate::BitReader;
#[doc = "Field `MBS` writer - Memory buffer select"]
pub type MbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBURST` reader - Transfer burst type of peripheral"]
pub type PburstR = crate::FieldReader;
#[doc = "Field `PBURST` writer - Transfer burst type of peripheral"]
pub type PburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBURST` reader - Transfer burst type of memory"]
pub type MburstR = crate::FieldReader;
#[doc = "Field `MBURST` writer - Transfer burst type of memory"]
pub type MburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERIEN` reader - Peripheral enable"]
pub type PerienR = crate::FieldReader;
#[doc = "Field `PERIEN` writer - Peripheral enable"]
pub type PerienW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for single data mode exception interrupt"]
    #[inline(always)]
    pub fn sdeie(&self) -> SdeieR {
        SdeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    pub fn taeie(&self) -> TaeieR {
        TaeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HtfieR {
        HtfieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FtfieR {
        FtfieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transfer flow controller select"]
    #[inline(always)]
    pub fn tfcs(&self) -> TfcsR {
        TfcsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Transfer mode"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Circulation mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CmenR {
        CmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PnagaR {
        PnagaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MnagaR {
        MnagaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Transfer width of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PwidthR {
        PwidthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Transfer width of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MwidthR {
        MwidthR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral address increment fixed"]
    #[inline(always)]
    pub fn paif(&self) -> PaifR {
        PaifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Switch-buffer mode enable"]
    #[inline(always)]
    pub fn sbmen(&self) -> SbmenR {
        SbmenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Memory buffer select"]
    #[inline(always)]
    pub fn mbs(&self) -> MbsR {
        MbsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Transfer burst type of peripheral"]
    #[inline(always)]
    pub fn pburst(&self) -> PburstR {
        PburstR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Transfer burst type of memory"]
    #[inline(always)]
    pub fn mburst(&self) -> MburstR {
        MburstR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Peripheral enable"]
    #[inline(always)]
    pub fn perien(&self) -> PerienR {
        PerienR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<Ch4ctlSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable bit for single data mode exception interrupt"]
    #[inline(always)]
    pub fn sdeie(&mut self) -> SdeieW<Ch4ctlSpec> {
        SdeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    pub fn taeie(&mut self) -> TaeieW<Ch4ctlSpec> {
        TaeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&mut self) -> HtfieW<Ch4ctlSpec> {
        HtfieW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&mut self) -> FtfieW<Ch4ctlSpec> {
        FtfieW::new(self, 4)
    }
    #[doc = "Bit 5 - Transfer flow controller select"]
    #[inline(always)]
    pub fn tfcs(&mut self) -> TfcsW<Ch4ctlSpec> {
        TfcsW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Transfer mode"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<Ch4ctlSpec> {
        TmW::new(self, 6)
    }
    #[doc = "Bit 8 - Circulation mode enable"]
    #[inline(always)]
    pub fn cmen(&mut self) -> CmenW<Ch4ctlSpec> {
        CmenW::new(self, 8)
    }
    #[doc = "Bit 9 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&mut self) -> PnagaW<Ch4ctlSpec> {
        PnagaW::new(self, 9)
    }
    #[doc = "Bit 10 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&mut self) -> MnagaW<Ch4ctlSpec> {
        MnagaW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Transfer width of peripheral"]
    #[inline(always)]
    pub fn pwidth(&mut self) -> PwidthW<Ch4ctlSpec> {
        PwidthW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Transfer width of memory"]
    #[inline(always)]
    pub fn mwidth(&mut self) -> MwidthW<Ch4ctlSpec> {
        MwidthW::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral address increment fixed"]
    #[inline(always)]
    pub fn paif(&mut self) -> PaifW<Ch4ctlSpec> {
        PaifW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn prio(&mut self) -> PrioW<Ch4ctlSpec> {
        PrioW::new(self, 16)
    }
    #[doc = "Bit 18 - Switch-buffer mode enable"]
    #[inline(always)]
    pub fn sbmen(&mut self) -> SbmenW<Ch4ctlSpec> {
        SbmenW::new(self, 18)
    }
    #[doc = "Bit 19 - Memory buffer select"]
    #[inline(always)]
    pub fn mbs(&mut self) -> MbsW<Ch4ctlSpec> {
        MbsW::new(self, 19)
    }
    #[doc = "Bits 21:22 - Transfer burst type of peripheral"]
    #[inline(always)]
    pub fn pburst(&mut self) -> PburstW<Ch4ctlSpec> {
        PburstW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Transfer burst type of memory"]
    #[inline(always)]
    pub fn mburst(&mut self) -> MburstW<Ch4ctlSpec> {
        MburstW::new(self, 23)
    }
    #[doc = "Bits 25:27 - Peripheral enable"]
    #[inline(always)]
    pub fn perien(&mut self) -> PerienW<Ch4ctlSpec> {
        PerienW::new(self, 25)
    }
}
#[doc = "Channel 4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4ctlSpec;
impl crate::RegisterSpec for Ch4ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4ctl::R`](R) reader structure"]
impl crate::Readable for Ch4ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4ctl::W`](W) writer structure"]
impl crate::Writable for Ch4ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4CTL to value 0"]
impl crate::Resettable for Ch4ctlSpec {}
