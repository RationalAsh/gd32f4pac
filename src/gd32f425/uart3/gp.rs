#[doc = "Register `GP` reader"]
pub type R = crate::R<GpSpec>;
#[doc = "Register `GP` writer"]
pub type W = crate::W<GpSpec>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GUAT` reader - Guard time value in Smartcard mode"]
pub type GuatR = crate::FieldReader;
#[doc = "Field `GUAT` writer - Guard time value in Smartcard mode"]
pub type GuatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value in Smartcard mode"]
    #[inline(always)]
    pub fn guat(&self) -> GuatR {
        GuatR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<GpSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value in Smartcard mode"]
    #[inline(always)]
    pub fn guat(&mut self) -> GuatW<GpSpec> {
        GuatW::new(self, 8)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpSpec;
impl crate::RegisterSpec for GpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GpSpec {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GP to value 0"]
impl crate::Resettable for GpSpec {}
