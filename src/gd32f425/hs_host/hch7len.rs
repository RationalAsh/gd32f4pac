#[doc = "Register `HCH7LEN` reader"]
pub type R = crate::R<Hch7lenSpec>;
#[doc = "Register `HCH7LEN` writer"]
pub type W = crate::W<Hch7lenSpec>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TlenR = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PcntR = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DpidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PING` reader - Ping token request"]
pub type PingR = crate::BitReader;
#[doc = "Field `PING` writer - Ping token request"]
pub type PingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TlenR {
        TlenR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Ping token request"]
    #[inline(always)]
    pub fn ping(&self) -> PingR {
        PingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TlenW<Hch7lenSpec> {
        TlenW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<Hch7lenSpec> {
        PcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&mut self) -> DpidW<Hch7lenSpec> {
        DpidW::new(self, 29)
    }
    #[doc = "Bit 31 - Ping token request"]
    #[inline(always)]
    pub fn ping(&mut self) -> PingW<Hch7lenSpec> {
        PingW::new(self, 31)
    }
}
#[doc = "host channel-7 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`hch7len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch7len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch7lenSpec;
impl crate::RegisterSpec for Hch7lenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch7len::R`](R) reader structure"]
impl crate::Readable for Hch7lenSpec {}
#[doc = "`write(|w| ..)` method takes [`hch7len::W`](W) writer structure"]
impl crate::Writable for Hch7lenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH7LEN to value 0"]
impl crate::Resettable for Hch7lenSpec {}
