#[doc = "Register `BKP13` reader"]
pub type R = crate::R<Bkp13Spec>;
#[doc = "Register `BKP13` writer"]
pub type W = crate::W<Bkp13Spec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Bkp13Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp13Spec;
impl crate::RegisterSpec for Bkp13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp13::R`](R) reader structure"]
impl crate::Readable for Bkp13Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp13::W`](W) writer structure"]
impl crate::Writable for Bkp13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP13 to value 0"]
impl crate::Resettable for Bkp13Spec {}
