#[doc = "Register `CH3CV` reader"]
pub type R = crate::R<Ch3cvSpec>;
#[doc = "Register `CH3CV` writer"]
pub type W = crate::W<Ch3cvSpec>;
#[doc = "Field `CH3VAL` reader - Capture or compare value of channel 3"]
pub type Ch3valR = crate::FieldReader<u16>;
#[doc = "Field `CH3VAL` writer - Capture or compare value of channel 3"]
pub type Ch3valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    pub fn ch3val(&self) -> Ch3valR {
        Ch3valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    pub fn ch3val(&mut self) -> Ch3valW<Ch3cvSpec> {
        Ch3valW::new(self, 0)
    }
}
#[doc = "Channel 3 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3cvSpec;
impl crate::RegisterSpec for Ch3cvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cv::R`](R) reader structure"]
impl crate::Readable for Ch3cvSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3cv::W`](W) writer structure"]
impl crate::Writable for Ch3cvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CV to value 0"]
impl crate::Resettable for Ch3cvSpec {}
