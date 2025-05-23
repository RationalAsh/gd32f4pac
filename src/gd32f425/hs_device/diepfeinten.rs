#[doc = "Register `DIEPFEINTEN` reader"]
pub type R = crate::R<DiepfeintenSpec>;
#[doc = "Register `DIEPFEINTEN` writer"]
pub type W = crate::W<DiepfeintenSpec>;
#[doc = "Field `IEPTXFEIE` reader - IN EP Tx FIFO empty interrupt enable bits"]
pub type IeptxfeieR = crate::FieldReader;
#[doc = "Field `IEPTXFEIE` writer - IN EP Tx FIFO empty interrupt enable bits"]
pub type IeptxfeieW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&self) -> IeptxfeieR {
        IeptxfeieR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&mut self) -> IeptxfeieW<DiepfeintenSpec> {
        IeptxfeieW::new(self, 0)
    }
}
#[doc = "device IN endpoint FIFO empty interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepfeinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepfeinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepfeintenSpec;
impl crate::RegisterSpec for DiepfeintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepfeinten::R`](R) reader structure"]
impl crate::Readable for DiepfeintenSpec {}
#[doc = "`write(|w| ..)` method takes [`diepfeinten::W`](W) writer structure"]
impl crate::Writable for DiepfeintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPFEINTEN to value 0"]
impl crate::Resettable for DiepfeintenSpec {}
