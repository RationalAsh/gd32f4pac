#[doc = "Register `PTP_ETL` reader"]
pub type R = crate::R<PtpEtlSpec>;
#[doc = "Register `PTP_ETL` writer"]
pub type W = crate::W<PtpEtlSpec>;
#[doc = "Field `ETSL` reader - Expected time stamp low"]
pub type EtslR = crate::FieldReader<u32>;
#[doc = "Field `ETSL` writer - Expected time stamp low"]
pub type EtslW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    pub fn etsl(&self) -> EtslR {
        EtslR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    pub fn etsl(&mut self) -> EtslW<PtpEtlSpec> {
        EtslW::new(self, 0)
    }
}
#[doc = "Ethernet PTP expected time low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptp_etl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptp_etl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpEtlSpec;
impl crate::RegisterSpec for PtpEtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_etl::R`](R) reader structure"]
impl crate::Readable for PtpEtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_etl::W`](W) writer structure"]
impl crate::Writable for PtpEtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTP_ETL to value 0"]
impl crate::Resettable for PtpEtlSpec {}
