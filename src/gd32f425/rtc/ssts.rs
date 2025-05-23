#[doc = "Register `SSTS` reader"]
pub type R = crate::R<SstsSpec>;
#[doc = "Field `SSC` reader - Sub second value"]
pub type SscR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Sub second of time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstsSpec;
impl crate::RegisterSpec for SstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssts::R`](R) reader structure"]
impl crate::Readable for SstsSpec {}
#[doc = "`reset()` method sets SSTS to value 0"]
impl crate::Resettable for SstsSpec {}
