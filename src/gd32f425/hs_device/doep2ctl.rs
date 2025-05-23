#[doc = "Register `DOEP2CTL` reader"]
pub type R = crate::R<Doep2ctlSpec>;
#[doc = "Register `DOEP2CTL` writer"]
pub type W = crate::W<Doep2ctlSpec>;
#[doc = "Field `MPL` reader - maximum packet length"]
pub type MplR = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - maximum packet length"]
pub type MplW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPACT` reader - Endpoint active"]
pub type EpactR = crate::BitReader;
#[doc = "Field `EPACT` writer - Endpoint active"]
pub type EpactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOFRM_DPID` reader - EOFRM/DPID"]
pub type EofrmDpidR = crate::BitReader;
#[doc = "Field `NAKS` reader - NAK status"]
pub type NaksR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNOOP` reader - Snoop mode"]
pub type SnoopR = crate::BitReader;
#[doc = "Field `SNOOP` writer - Snoop mode"]
pub type SnoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID_SEVENFRM` writer - SD0PID/SEVENFRM"]
pub type Sd0pidSevenfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1PID_SODDFRM` writer - SD1PID/SODDFRM"]
pub type Sd1pidSoddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPD` reader - Endpoint disable"]
pub type EpdR = crate::BitReader;
#[doc = "Field `EPD` writer - Endpoint disable"]
pub type EpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN` reader - Endpoint enable"]
pub type EpenR = crate::BitReader;
#[doc = "Field `EPEN` writer - Endpoint enable"]
pub type EpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    pub fn mpl(&self) -> MplR {
        MplR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    pub fn epact(&self) -> EpactR {
        EpactR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EOFRM/DPID"]
    #[inline(always)]
    pub fn eofrm_dpid(&self) -> EofrmDpidR {
        EofrmDpidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naks(&self) -> NaksR {
        NaksR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snoop(&self) -> SnoopR {
        SnoopR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epd(&self) -> EpdR {
        EpdR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epen(&self) -> EpenR {
        EpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    pub fn mpl(&mut self) -> MplW<Doep2ctlSpec> {
        MplW::new(self, 0)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    pub fn epact(&mut self) -> EpactW<Doep2ctlSpec> {
        EpactW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<Doep2ctlSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snoop(&mut self) -> SnoopW<Doep2ctlSpec> {
        SnoopW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<Doep2ctlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<Doep2ctlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<Doep2ctlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - SD0PID/SEVENFRM"]
    #[inline(always)]
    pub fn sd0pid_sevenfrm(&mut self) -> Sd0pidSevenfrmW<Doep2ctlSpec> {
        Sd0pidSevenfrmW::new(self, 28)
    }
    #[doc = "Bit 29 - SD1PID/SODDFRM"]
    #[inline(always)]
    pub fn sd1pid_soddfrm(&mut self) -> Sd1pidSoddfrmW<Doep2ctlSpec> {
        Sd1pidSoddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epd(&mut self) -> EpdW<Doep2ctlSpec> {
        EpdW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epen(&mut self) -> EpenW<Doep2ctlSpec> {
        EpenW::new(self, 31)
    }
}
#[doc = "Device OUT endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep2ctlSpec;
impl crate::RegisterSpec for Doep2ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2ctl::R`](R) reader structure"]
impl crate::Readable for Doep2ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`doep2ctl::W`](W) writer structure"]
impl crate::Writable for Doep2ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP2CTL to value 0"]
impl crate::Resettable for Doep2ctlSpec {}
