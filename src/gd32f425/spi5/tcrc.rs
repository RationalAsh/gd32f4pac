#[doc = "Register `TCRC` reader"]
pub type R = crate::R<TcrcSpec>;
#[doc = "Field `TCR` reader - Tx CRC register"]
pub type TcrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrcSpec;
impl crate::RegisterSpec for TcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcrc::R`](R) reader structure"]
impl crate::Readable for TcrcSpec {}
#[doc = "`reset()` method sets TCRC to value 0"]
impl crate::Resettable for TcrcSpec {}
