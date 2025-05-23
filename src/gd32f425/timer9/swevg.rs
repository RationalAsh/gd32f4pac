#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Field `UPG` writer - Update generation"]
pub type UpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type Ch0gW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UpgW<SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> Ch0gW<SwevgSpec> {
        Ch0gW::new(self, 1)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {}
