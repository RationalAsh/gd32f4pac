#[doc = "Register `PTP_SSINC` reader"]
pub type R = crate::R<PtpSsincSpec>;
#[doc = "Register `PTP_SSINC` writer"]
pub type W = crate::W<PtpSsincSpec>;
#[doc = "Field `STMSSI` reader - System time subsecond increment"]
pub type StmssiR = crate::FieldReader;
#[doc = "Field `STMSSI` writer - System time subsecond increment"]
pub type StmssiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stmssi(&self) -> StmssiR {
        StmssiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stmssi(&mut self) -> StmssiW<PtpSsincSpec> {
        StmssiW::new(self, 0)
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptp_ssinc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptp_ssinc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpSsincSpec;
impl crate::RegisterSpec for PtpSsincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_ssinc::R`](R) reader structure"]
impl crate::Readable for PtpSsincSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_ssinc::W`](W) writer structure"]
impl crate::Writable for PtpSsincSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTP_SSINC to value 0"]
impl crate::Resettable for PtpSsincSpec {}
