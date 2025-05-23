#[doc = "Register `ADDAPB1RST` reader"]
pub type R = crate::R<Addapb1rstSpec>;
#[doc = "Register `ADDAPB1RST` writer"]
pub type W = crate::W<Addapb1rstSpec>;
#[doc = "CTC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctcrst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Ctcrst> for bool {
    #[inline(always)]
    fn from(variant: Ctcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCRST` reader - CTC reset"]
pub type CtcrstR = crate::BitReader<Ctcrst>;
impl CtcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctcrst> {
        match self.bits {
            true => Some(Ctcrst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctcrst::Reset
    }
}
#[doc = "Field `CTCRST` writer - CTC reset"]
pub type CtcrstW<'a, REG> = crate::BitWriter<'a, REG, Ctcrst>;
impl<'a, REG> CtcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcrst::Reset)
    }
}
#[doc = "Field `IREFRST` reader - IREF reset"]
pub use CtcrstR as IrefrstR;
#[doc = "Field `IREFRST` writer - IREF reset"]
pub use CtcrstW as IrefrstW;
impl R {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CtcrstR {
        CtcrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - IREF reset"]
    #[inline(always)]
    pub fn irefrst(&self) -> IrefrstR {
        IrefrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CtcrstW<Addapb1rstSpec> {
        CtcrstW::new(self, 27)
    }
    #[doc = "Bit 31 - IREF reset"]
    #[inline(always)]
    pub fn irefrst(&mut self) -> IrefrstW<Addapb1rstSpec> {
        IrefrstW::new(self, 31)
    }
}
#[doc = "APB1 additional reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`addapb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addapb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1rstSpec;
impl crate::RegisterSpec for Addapb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1rst::R`](R) reader structure"]
impl crate::Readable for Addapb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1rst::W`](W) writer structure"]
impl crate::Writable for Addapb1rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for Addapb1rstSpec {}
