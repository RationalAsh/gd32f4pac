#[doc = "Register `SAMPT1` reader"]
pub type R = crate::R<Sampt1Spec>;
#[doc = "Register `SAMPT1` writer"]
pub type W = crate::W<Sampt1Spec>;
#[doc = "Channel 0 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spt0 {
    #[doc = "0: Channel sample time is 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: Channel sample time is 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: Channel sample time is 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: Channel sample time is 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: Channel sample time is 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: Channel sample time is 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: Channel sample time is 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: Channel sample time is 480 cycles"]
    Cycles480 = 7,
}
impl From<Spt0> for u8 {
    #[inline(always)]
    fn from(variant: Spt0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spt0 {
    type Ux = u8;
}
impl crate::IsEnum for Spt0 {}
#[doc = "Field `SPT0` reader - Channel 0 sample time selection"]
pub type Spt0R = crate::FieldReader<Spt0>;
impl Spt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt0 {
        match self.bits {
            0 => Spt0::Cycles3,
            1 => Spt0::Cycles15,
            2 => Spt0::Cycles28,
            3 => Spt0::Cycles56,
            4 => Spt0::Cycles84,
            5 => Spt0::Cycles112,
            6 => Spt0::Cycles144,
            7 => Spt0::Cycles480,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel sample time is 3 cycles"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == Spt0::Cycles3
    }
    #[doc = "Channel sample time is 15 cycles"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == Spt0::Cycles15
    }
    #[doc = "Channel sample time is 28 cycles"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == Spt0::Cycles28
    }
    #[doc = "Channel sample time is 56 cycles"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == Spt0::Cycles56
    }
    #[doc = "Channel sample time is 84 cycles"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == Spt0::Cycles84
    }
    #[doc = "Channel sample time is 112 cycles"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == Spt0::Cycles112
    }
    #[doc = "Channel sample time is 144 cycles"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == Spt0::Cycles144
    }
    #[doc = "Channel sample time is 480 cycles"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == Spt0::Cycles480
    }
}
#[doc = "Field `SPT0` writer - Channel 0 sample time selection"]
pub type Spt0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Spt0, crate::Safe>;
impl<'a, REG> Spt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel sample time is 3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles3)
    }
    #[doc = "Channel sample time is 15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles15)
    }
    #[doc = "Channel sample time is 28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles28)
    }
    #[doc = "Channel sample time is 56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles56)
    }
    #[doc = "Channel sample time is 84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles84)
    }
    #[doc = "Channel sample time is 112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles112)
    }
    #[doc = "Channel sample time is 144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles144)
    }
    #[doc = "Channel sample time is 480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles480)
    }
}
#[doc = "Field `SPT1` reader - Channel 1 sample time selection"]
pub use Spt0R as Spt1R;
#[doc = "Field `SPT2` reader - Channel 2 sample time selection"]
pub use Spt0R as Spt2R;
#[doc = "Field `SPT3` reader - Channel 3 sample time selection"]
pub use Spt0R as Spt3R;
#[doc = "Field `SPT4` reader - Channel 4 sample time selection"]
pub use Spt0R as Spt4R;
#[doc = "Field `SPT5` reader - Channel 5 sample time selection"]
pub use Spt0R as Spt5R;
#[doc = "Field `SPT6` reader - Channel 6 sample time selection"]
pub use Spt0R as Spt6R;
#[doc = "Field `SPT7` reader - Channel 7 sample time selection"]
pub use Spt0R as Spt7R;
#[doc = "Field `SPT8` reader - Channel 8 sample time selection"]
pub use Spt0R as Spt8R;
#[doc = "Field `SPT9` reader - Channel 9 sample time selection"]
pub use Spt0R as Spt9R;
#[doc = "Field `SPT1` writer - Channel 1 sample time selection"]
pub use Spt0W as Spt1W;
#[doc = "Field `SPT2` writer - Channel 2 sample time selection"]
pub use Spt0W as Spt2W;
#[doc = "Field `SPT3` writer - Channel 3 sample time selection"]
pub use Spt0W as Spt3W;
#[doc = "Field `SPT4` writer - Channel 4 sample time selection"]
pub use Spt0W as Spt4W;
#[doc = "Field `SPT5` writer - Channel 5 sample time selection"]
pub use Spt0W as Spt5W;
#[doc = "Field `SPT6` writer - Channel 6 sample time selection"]
pub use Spt0W as Spt6W;
#[doc = "Field `SPT7` writer - Channel 7 sample time selection"]
pub use Spt0W as Spt7W;
#[doc = "Field `SPT8` writer - Channel 8 sample time selection"]
pub use Spt0W as Spt8W;
#[doc = "Field `SPT9` writer - Channel 9 sample time selection"]
pub use Spt0W as Spt9W;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&self) -> Spt0R {
        Spt0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&self) -> Spt1R {
        Spt1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&self) -> Spt2R {
        Spt2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&self) -> Spt3R {
        Spt3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&self) -> Spt4R {
        Spt4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&self) -> Spt5R {
        Spt5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&self) -> Spt6R {
        Spt6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&self) -> Spt7R {
        Spt7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&self) -> Spt8R {
        Spt8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&self) -> Spt9R {
        Spt9R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&mut self) -> Spt0W<Sampt1Spec> {
        Spt0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&mut self) -> Spt1W<Sampt1Spec> {
        Spt1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&mut self) -> Spt2W<Sampt1Spec> {
        Spt2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&mut self) -> Spt3W<Sampt1Spec> {
        Spt3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&mut self) -> Spt4W<Sampt1Spec> {
        Spt4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&mut self) -> Spt5W<Sampt1Spec> {
        Spt5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&mut self) -> Spt6W<Sampt1Spec> {
        Spt6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&mut self) -> Spt7W<Sampt1Spec> {
        Spt7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&mut self) -> Spt8W<Sampt1Spec> {
        Spt8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&mut self) -> Spt9W<Sampt1Spec> {
        Spt9W::new(self, 27)
    }
}
#[doc = "Sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sampt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sampt1Spec;
impl crate::RegisterSpec for Sampt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt1::R`](R) reader structure"]
impl crate::Readable for Sampt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sampt1::W`](W) writer structure"]
impl crate::Writable for Sampt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPT1 to value 0"]
impl crate::Resettable for Sampt1Spec {}
