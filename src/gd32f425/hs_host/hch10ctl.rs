#[doc = "Register `HCH10CTL` reader"]
pub type R = crate::R<Hch10ctlSpec>;
#[doc = "Register `HCH10CTL` writer"]
pub type W = crate::W<Hch10ctlSpec>;
#[doc = "Field `MPL` reader - Maximum packet size"]
pub type MplR = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - Maximum packet size"]
pub type MplW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EpdirR = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSD` reader - Low-speed device"]
pub type LsdR = crate::BitReader;
#[doc = "Field `LSD` writer - Low-speed device"]
pub type LsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MPC` reader - Multiple packet count"]
pub type MpcR = crate::FieldReader;
#[doc = "Field `MPC` writer - Multiple packet count"]
pub type MpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAR` reader - Device address"]
pub type DarR = crate::FieldReader;
#[doc = "Field `DAR` writer - Device address"]
pub type DarW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type OddfrmR = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type OddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIS` reader - Channel disable"]
pub type CdisR = crate::BitReader;
#[doc = "Field `CDIS` writer - Channel disable"]
pub type CdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Channel enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Channel enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpl(&self) -> MplR {
        MplR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsd(&self) -> LsdR {
        LsdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multiple packet count"]
    #[inline(always)]
    pub fn mpc(&self) -> MpcR {
        MpcR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> OddfrmR {
        OddfrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn cdis(&self) -> CdisR {
        CdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpl(&mut self) -> MplW<Hch10ctlSpec> {
        MplW::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EpnumW<Hch10ctlSpec> {
        EpnumW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EpdirW<Hch10ctlSpec> {
        EpdirW::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsd(&mut self) -> LsdW<Hch10ctlSpec> {
        LsdW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<Hch10ctlSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multiple packet count"]
    #[inline(always)]
    pub fn mpc(&mut self) -> MpcW<Hch10ctlSpec> {
        MpcW::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<Hch10ctlSpec> {
        DarW::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> OddfrmW<Hch10ctlSpec> {
        OddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn cdis(&mut self) -> CdisW<Hch10ctlSpec> {
        CdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<Hch10ctlSpec> {
        CenW::new(self, 31)
    }
}
#[doc = "host channel-10 control register (HCH10CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch10ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch10ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch10ctlSpec;
impl crate::RegisterSpec for Hch10ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch10ctl::R`](R) reader structure"]
impl crate::Readable for Hch10ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hch10ctl::W`](W) writer structure"]
impl crate::Writable for Hch10ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH10CTL to value 0"]
impl crate::Resettable for Hch10ctlSpec {}
