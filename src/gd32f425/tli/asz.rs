#[doc = "Register `ASZ` reader"]
pub type R = crate::R<AszSpec>;
#[doc = "Register `ASZ` writer"]
pub type W = crate::W<AszSpec>;
#[doc = "Field `VASZ` reader - Size of the vertical active area width plus back porch and synchronous pulse"]
pub type VaszR = crate::FieldReader<u16>;
#[doc = "Field `VASZ` writer - Size of the vertical active area width plus back porch and synchronous pulse"]
pub type VaszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HASZ` reader - Size of the horizontal active area width plus back porch and synchronous pulse"]
pub type HaszR = crate::FieldReader<u16>;
#[doc = "Field `HASZ` writer - Size of the horizontal active area width plus back porch and synchronous pulse"]
pub type HaszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Size of the vertical active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn vasz(&self) -> VaszR {
        VaszR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Size of the horizontal active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn hasz(&self) -> HaszR {
        HaszR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Size of the vertical active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn vasz(&mut self) -> VaszW<AszSpec> {
        VaszW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Size of the horizontal active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn hasz(&mut self) -> HaszW<AszSpec> {
        HaszW::new(self, 16)
    }
}
#[doc = "Active size register\n\nYou can [`read`](crate::Reg::read) this register and get [`asz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AszSpec;
impl crate::RegisterSpec for AszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asz::R`](R) reader structure"]
impl crate::Readable for AszSpec {}
#[doc = "`write(|w| ..)` method takes [`asz::W`](W) writer structure"]
impl crate::Writable for AszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASZ to value 0"]
impl crate::Resettable for AszSpec {}
