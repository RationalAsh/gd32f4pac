#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Port x configuration bits (x = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0 {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: General purpose output mode"]
    Output = 1,
    #[doc = "2: Alternate function mode"]
    Alternate = 2,
    #[doc = "3: Analog mode"]
    Analog = 3,
}
impl From<Ctl0> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0 {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0 {}
#[doc = "Field `CTL0` reader - Port x configuration bits (x = 0)"]
pub type Ctl0R = crate::FieldReader<Ctl0>;
impl Ctl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0 {
        match self.bits {
            0 => Ctl0::Input,
            1 => Ctl0::Output,
            2 => Ctl0::Alternate,
            3 => Ctl0::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Ctl0::Input
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Ctl0::Output
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == Ctl0::Alternate
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == Ctl0::Analog
    }
}
#[doc = "Field `CTL0` writer - Port x configuration bits (x = 0)"]
pub type Ctl0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl0, crate::Safe>;
impl<'a, REG> Ctl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0::Input)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0::Output)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0::Alternate)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0::Analog)
    }
}
#[doc = "Field `CTL1` reader - Port x configuration bits (x = 1)"]
pub use Ctl0R as Ctl1R;
#[doc = "Field `CTL2` reader - Port x configuration bits (x = 2)"]
pub use Ctl0R as Ctl2R;
#[doc = "Field `CTL3` reader - Port x configuration bits (x = 3)"]
pub use Ctl0R as Ctl3R;
#[doc = "Field `CTL4` reader - Port x configuration bits (x = 4 )"]
pub use Ctl0R as Ctl4R;
#[doc = "Field `CTL5` reader - Port x configuration bits (x = 5)"]
pub use Ctl0R as Ctl5R;
#[doc = "Field `CTL6` reader - Port x configuration bits (x = 6 )"]
pub use Ctl0R as Ctl6R;
#[doc = "Field `CTL7` reader - Port x configuration bits (x = 7)"]
pub use Ctl0R as Ctl7R;
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub use Ctl0R as Ctl8R;
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub use Ctl0R as Ctl9R;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub use Ctl0R as Ctl10R;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub use Ctl0R as Ctl11R;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub use Ctl0R as Ctl12R;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub use Ctl0R as Ctl13R;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub use Ctl0R as Ctl14R;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub use Ctl0R as Ctl15R;
#[doc = "Field `CTL1` writer - Port x configuration bits (x = 1)"]
pub use Ctl0W as Ctl1W;
#[doc = "Field `CTL2` writer - Port x configuration bits (x = 2)"]
pub use Ctl0W as Ctl2W;
#[doc = "Field `CTL3` writer - Port x configuration bits (x = 3)"]
pub use Ctl0W as Ctl3W;
#[doc = "Field `CTL4` writer - Port x configuration bits (x = 4 )"]
pub use Ctl0W as Ctl4W;
#[doc = "Field `CTL5` writer - Port x configuration bits (x = 5)"]
pub use Ctl0W as Ctl5W;
#[doc = "Field `CTL6` writer - Port x configuration bits (x = 6 )"]
pub use Ctl0W as Ctl6W;
#[doc = "Field `CTL7` writer - Port x configuration bits (x = 7)"]
pub use Ctl0W as Ctl7W;
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub use Ctl0W as Ctl8W;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub use Ctl0W as Ctl9W;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub use Ctl0W as Ctl10W;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub use Ctl0W as Ctl11W;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub use Ctl0W as Ctl12W;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub use Ctl0W as Ctl13W;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub use Ctl0W as Ctl14W;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub use Ctl0W as Ctl15W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> Ctl0R {
        Ctl0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> Ctl1R {
        Ctl1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> Ctl2R {
        Ctl2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> Ctl3R {
        Ctl3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (x = 4 )"]
    #[inline(always)]
    pub fn ctl4(&self) -> Ctl4R {
        Ctl4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> Ctl5R {
        Ctl5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (x = 6 )"]
    #[inline(always)]
    pub fn ctl6(&self) -> Ctl6R {
        Ctl6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> Ctl7R {
        Ctl7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> Ctl8R {
        Ctl8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> Ctl9R {
        Ctl9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> Ctl10R {
        Ctl10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> Ctl11R {
        Ctl11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> Ctl12R {
        Ctl12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> Ctl13R {
        Ctl13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> Ctl14R {
        Ctl14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> Ctl15R {
        Ctl15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&mut self) -> Ctl0W<CtlSpec> {
        Ctl0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&mut self) -> Ctl1W<CtlSpec> {
        Ctl1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&mut self) -> Ctl2W<CtlSpec> {
        Ctl2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&mut self) -> Ctl3W<CtlSpec> {
        Ctl3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (x = 4 )"]
    #[inline(always)]
    pub fn ctl4(&mut self) -> Ctl4W<CtlSpec> {
        Ctl4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&mut self) -> Ctl5W<CtlSpec> {
        Ctl5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (x = 6 )"]
    #[inline(always)]
    pub fn ctl6(&mut self) -> Ctl6W<CtlSpec> {
        Ctl6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&mut self) -> Ctl7W<CtlSpec> {
        Ctl7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&mut self) -> Ctl8W<CtlSpec> {
        Ctl8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&mut self) -> Ctl9W<CtlSpec> {
        Ctl9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&mut self) -> Ctl10W<CtlSpec> {
        Ctl10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&mut self) -> Ctl11W<CtlSpec> {
        Ctl11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&mut self) -> Ctl12W<CtlSpec> {
        Ctl12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&mut self) -> Ctl13W<CtlSpec> {
        Ctl13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&mut self) -> Ctl14W<CtlSpec> {
        Ctl14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&mut self) -> Ctl15W<CtlSpec> {
        Ctl15W::new(self, 30)
    }
}
#[doc = "GPIO port control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0x0280"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0280;
}
