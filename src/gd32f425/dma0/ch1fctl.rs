#[doc = "Register `CH1FCTL` reader"]
pub type R = crate::R<Ch1fctlSpec>;
#[doc = "Register `CH1FCTL` writer"]
pub type W = crate::W<Ch1fctlSpec>;
#[doc = "Field `FCCV` reader - FIFO counter critical value"]
pub type FccvR = crate::FieldReader;
#[doc = "Field `FCCV` writer - FIFO counter critical value"]
pub type FccvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MDMEN` reader - Multi-data mode enable"]
pub type MdmenR = crate::BitReader;
#[doc = "Field `MDMEN` writer - Multi-data mode enable"]
pub type MdmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCNT` reader - FIFO counter"]
pub type FcntR = crate::FieldReader;
#[doc = "Field `FCNT` writer - FIFO counter"]
pub type FcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FEEIE` reader - Enable bit for FIFO error and exception interrupt"]
pub type FeeieR = crate::BitReader;
#[doc = "Field `FEEIE` writer - Enable bit for FIFO error and exception interrupt"]
pub type FeeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO counter critical value"]
    #[inline(always)]
    pub fn fccv(&self) -> FccvR {
        FccvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Multi-data mode enable"]
    #[inline(always)]
    pub fn mdmen(&self) -> MdmenR {
        MdmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO counter"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - Enable bit for FIFO error and exception interrupt"]
    #[inline(always)]
    pub fn feeie(&self) -> FeeieR {
        FeeieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO counter critical value"]
    #[inline(always)]
    pub fn fccv(&mut self) -> FccvW<Ch1fctlSpec> {
        FccvW::new(self, 0)
    }
    #[doc = "Bit 2 - Multi-data mode enable"]
    #[inline(always)]
    pub fn mdmen(&mut self) -> MdmenW<Ch1fctlSpec> {
        MdmenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - FIFO counter"]
    #[inline(always)]
    pub fn fcnt(&mut self) -> FcntW<Ch1fctlSpec> {
        FcntW::new(self, 3)
    }
    #[doc = "Bit 7 - Enable bit for FIFO error and exception interrupt"]
    #[inline(always)]
    pub fn feeie(&mut self) -> FeeieW<Ch1fctlSpec> {
        FeeieW::new(self, 7)
    }
}
#[doc = "Channel 1 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1fctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1fctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1fctlSpec;
impl crate::RegisterSpec for Ch1fctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1fctl::R`](R) reader structure"]
impl crate::Readable for Ch1fctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1fctl::W`](W) writer structure"]
impl crate::Writable for Ch1fctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1FCTL to value 0"]
impl crate::Resettable for Ch1fctlSpec {}
