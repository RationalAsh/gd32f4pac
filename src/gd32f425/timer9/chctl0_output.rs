#[doc = "Register `CHCTL0_Output` reader"]
pub type R = crate::R<Chctl0OutputSpec>;
#[doc = "Register `CHCTL0_Output` writer"]
pub type W = crate::W<Chctl0OutputSpec>;
#[doc = "Field `CH0MS` reader - Channel 0 I/O mode selection"]
pub type Ch0msR = crate::FieldReader;
#[doc = "Field `CH0MS` writer - Channel 0 I/O mode selection"]
pub type Ch0msW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub type Ch0comfenR = crate::BitReader;
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub type Ch0comfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0COMSEN` reader - Channel 0 compare output shadow enable"]
pub type Ch0comsenR = crate::BitReader;
#[doc = "Field `CH0COMSEN` writer - Channel 0 compare output shadow enable"]
pub type Ch0comsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub type Ch0comctlR = crate::FieldReader;
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub type Ch0comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> Ch0msR {
        Ch0msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> Ch0comfenR {
        Ch0comfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> Ch0comsenR {
        Ch0comsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> Ch0comctlR {
        Ch0comctlR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> Ch0msW<Chctl0OutputSpec> {
        Ch0msW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> Ch0comfenW<Chctl0OutputSpec> {
        Ch0comfenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> Ch0comsenW<Chctl0OutputSpec> {
        Ch0comsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&mut self) -> Ch0comctlW<Chctl0OutputSpec> {
        Ch0comctlW::new(self, 4)
    }
}
#[doc = "Channel control register 0 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl0OutputSpec;
impl crate::RegisterSpec for Chctl0OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_output::R`](R) reader structure"]
impl crate::Readable for Chctl0OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl0_output::W`](W) writer structure"]
impl crate::Writable for Chctl0OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL0_Output to value 0"]
impl crate::Resettable for Chctl0OutputSpec {}
