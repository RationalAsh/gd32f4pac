#[doc = "Register `DAC0_R12DH` reader"]
pub type R = crate::R<Dac0R12dhSpec>;
#[doc = "Register `DAC0_R12DH` writer"]
pub type W = crate::W<Dac0R12dhSpec>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit right-aligned data"]
pub type Dac0DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit right-aligned data"]
pub type Dac0DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> Dac0DhR {
        Dac0DhR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> Dac0DhW<Dac0R12dhSpec> {
        Dac0DhW::new(self, 0)
    }
}
#[doc = "DAC0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_r12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_r12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0R12dhSpec;
impl crate::RegisterSpec for Dac0R12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_r12dh::R`](R) reader structure"]
impl crate::Readable for Dac0R12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0_r12dh::W`](W) writer structure"]
impl crate::Writable for Dac0R12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC0_R12DH to value 0"]
impl crate::Resettable for Dac0R12dhSpec {}
