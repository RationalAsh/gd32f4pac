#[doc = "Register `CH7CNT` reader"]
pub type R = crate::R<Ch7cntSpec>;
#[doc = "Register `CH7CNT` writer"]
pub type W = crate::W<Ch7cntSpec>;
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<Ch7cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Channel 7 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7cntSpec;
impl crate::RegisterSpec for Ch7cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7cnt::R`](R) reader structure"]
impl crate::Readable for Ch7cntSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7cnt::W`](W) writer structure"]
impl crate::Writable for Ch7cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7CNT to value 0"]
impl crate::Resettable for Ch7cntSpec {}
