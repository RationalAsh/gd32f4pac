#[doc = "Register `CH3CNT` reader"]
pub type R = crate::R<Ch3cntSpec>;
#[doc = "Register `CH3CNT` writer"]
pub type W = crate::W<Ch3cntSpec>;
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
    pub fn cnt(&mut self) -> CntW<Ch3cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Channel 3 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3cntSpec;
impl crate::RegisterSpec for Ch3cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cnt::R`](R) reader structure"]
impl crate::Readable for Ch3cntSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3cnt::W`](W) writer structure"]
impl crate::Writable for Ch3cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CNT to value 0"]
impl crate::Resettable for Ch3cntSpec {}
