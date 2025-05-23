#[doc = "Register `RCRC` reader"]
pub type R = crate::R<RcrcSpec>;
#[doc = "Field `RCR` reader - RX CRC register"]
pub type RcrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC register"]
    #[inline(always)]
    pub fn rcr(&self) -> RcrR {
        RcrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrcSpec;
impl crate::RegisterSpec for RcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcrc::R`](R) reader structure"]
impl crate::Readable for RcrcSpec {}
#[doc = "`reset()` method sets RCRC to value 0"]
impl crate::Resettable for RcrcSpec {}
