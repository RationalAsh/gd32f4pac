#[doc = "Register `RFIFOMP0` reader"]
pub type R = crate::R<Rfifomp0Spec>;
#[doc = "Field `DLENC` reader - Data length code"]
pub type DlencR = crate::FieldReader;
#[doc = "Field `FI` reader - Filtering index"]
pub type FiR = crate::FieldReader;
#[doc = "Field `TS` reader - Time stamp"]
pub type TsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DlencR {
        DlencR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Filtering index"]
    #[inline(always)]
    pub fn fi(&self) -> FiR {
        FiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO0 mailbox property register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifomp0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfifomp0Spec;
impl crate::RegisterSpec for Rfifomp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifomp0::R`](R) reader structure"]
impl crate::Readable for Rfifomp0Spec {}
#[doc = "`reset()` method sets RFIFOMP0 to value 0"]
impl crate::Resettable for Rfifomp0Spec {}
