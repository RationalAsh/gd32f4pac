#[doc = "Register `GAHBCS` reader"]
pub type R = crate::R<GahbcsSpec>;
#[doc = "Register `GAHBCS` writer"]
pub type W = crate::W<GahbcsSpec>;
#[doc = "Field `GINTEN` reader - Global interrupt enable"]
pub type GintenR = crate::BitReader;
#[doc = "Field `GINTEN` writer - Global interrupt enable"]
pub type GintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTH` reader - Tx FIFO threshold"]
pub type TxfthR = crate::BitReader;
#[doc = "Field `TXFTH` writer - Tx FIFO threshold"]
pub type TxfthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFTH` reader - Periodic Tx FIFO threshold"]
pub type PtxfthR = crate::BitReader;
#[doc = "Field `PTXFTH` writer - Periodic Tx FIFO threshold"]
pub type PtxfthW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&self) -> GintenR {
        GintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&self) -> TxfthR {
        TxfthR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&self) -> PtxfthR {
        PtxfthR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&mut self) -> GintenW<GahbcsSpec> {
        GintenW::new(self, 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&mut self) -> TxfthW<GahbcsSpec> {
        TxfthW::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&mut self) -> PtxfthW<GahbcsSpec> {
        PtxfthW::new(self, 8)
    }
}
#[doc = "Global AHB control and status register (USBFS_GAHBCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcsSpec;
impl crate::RegisterSpec for GahbcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcs::R`](R) reader structure"]
impl crate::Readable for GahbcsSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcs::W`](W) writer structure"]
impl crate::Writable for GahbcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAHBCS to value 0"]
impl crate::Resettable for GahbcsSpec {}
