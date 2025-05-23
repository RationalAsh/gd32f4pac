#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TLIEN` reader - TLI enable bit"]
pub type TlienR = crate::BitReader;
#[doc = "Field `TLIEN` writer - TLI enable bit"]
pub type TlienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDB` reader - Blue channel Dither Bits Number"]
pub type BdbR = crate::FieldReader;
#[doc = "Field `BDB` writer - Blue channel Dither Bits Number"]
pub type BdbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GDB` reader - Green channel Dither Bits Number"]
pub type GdbR = crate::FieldReader;
#[doc = "Field `GDB` writer - Green channel Dither Bits Number"]
pub type GdbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDB` reader - Red channel Dither Bits Number"]
pub type RdbR = crate::FieldReader;
#[doc = "Field `RDB` writer - Red channel Dither Bits Number"]
pub type RdbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DFEN` reader - Dither Function Enable"]
pub type DfenR = crate::BitReader;
#[doc = "Field `DFEN` writer - Dither Function Enable"]
pub type DfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPS` reader - Pixel Clock Polarity Selection"]
pub type ClkpsR = crate::BitReader;
#[doc = "Field `CLKPS` writer - Pixel Clock Polarity Selection"]
pub type ClkpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEPS` reader - Data Enable Polarity Selection"]
pub type DepsR = crate::BitReader;
#[doc = "Field `DEPS` writer - Data Enable Polarity Selection"]
pub type DepsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPPS` reader - Vertical Pulse Polarity Selection"]
pub type VppsR = crate::BitReader;
#[doc = "Field `VPPS` writer - Vertical Pulse Polarity Selection"]
pub type VppsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPPS` reader - Horizontal Pulse Polarity Selection"]
pub type HppsR = crate::BitReader;
#[doc = "Field `HPPS` writer - Horizontal Pulse Polarity Selection"]
pub type HppsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    pub fn tlien(&self) -> TlienR {
        TlienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Blue channel Dither Bits Number"]
    #[inline(always)]
    pub fn bdb(&self) -> BdbR {
        BdbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Green channel Dither Bits Number"]
    #[inline(always)]
    pub fn gdb(&self) -> GdbR {
        GdbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Red channel Dither Bits Number"]
    #[inline(always)]
    pub fn rdb(&self) -> RdbR {
        RdbR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DfenR {
        DfenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    pub fn clkps(&self) -> ClkpsR {
        ClkpsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    pub fn deps(&self) -> DepsR {
        DepsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    pub fn vpps(&self) -> VppsR {
        VppsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    pub fn hpps(&self) -> HppsR {
        HppsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    pub fn tlien(&mut self) -> TlienW<CtlSpec> {
        TlienW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Blue channel Dither Bits Number"]
    #[inline(always)]
    pub fn bdb(&mut self) -> BdbW<CtlSpec> {
        BdbW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Green channel Dither Bits Number"]
    #[inline(always)]
    pub fn gdb(&mut self) -> GdbW<CtlSpec> {
        GdbW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Red channel Dither Bits Number"]
    #[inline(always)]
    pub fn rdb(&mut self) -> RdbW<CtlSpec> {
        RdbW::new(self, 12)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DfenW<CtlSpec> {
        DfenW::new(self, 16)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    pub fn clkps(&mut self) -> ClkpsW<CtlSpec> {
        ClkpsW::new(self, 28)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    pub fn deps(&mut self) -> DepsW<CtlSpec> {
        DepsW::new(self, 29)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    pub fn vpps(&mut self) -> VppsW<CtlSpec> {
        VppsW::new(self, 30)
    }
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    pub fn hpps(&mut self) -> HppsW<CtlSpec> {
        HppsW::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x2220"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x2220;
}
