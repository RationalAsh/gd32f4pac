#[doc = "Register `NPATCFG3` reader"]
pub type R = crate::R<Npatcfg3Spec>;
#[doc = "Register `NPATCFG3` writer"]
pub type W = crate::W<Npatcfg3Spec>;
#[doc = "Field `ATTSET` reader - Attribute memory setup time"]
pub type AttsetR = crate::FieldReader;
#[doc = "Field `ATTSET` writer - Attribute memory setup time"]
pub type AttsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAIT` reader - Attribute memory wait time"]
pub type AttwaitR = crate::FieldReader;
#[doc = "Field `ATTWAIT` writer - Attribute memory wait time"]
pub type AttwaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHLD` reader - Attribute memory hold time"]
pub type AtthldR = crate::FieldReader;
#[doc = "Field `ATTHLD` writer - Attribute memory hold time"]
pub type AtthldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZ` reader - Attribute memory data bus HiZ time"]
pub type AtthizR = crate::FieldReader;
#[doc = "Field `ATTHIZ` writer - Attribute memory data bus HiZ time"]
pub type AtthizW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&self) -> AttsetR {
        AttsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> AttwaitR {
        AttwaitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&self) -> AtthldR {
        AtthldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> AtthizR {
        AtthizR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&mut self) -> AttsetW<Npatcfg3Spec> {
        AttsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&mut self) -> AttwaitW<Npatcfg3Spec> {
        AttwaitW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&mut self) -> AtthldW<Npatcfg3Spec> {
        AtthldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&mut self) -> AtthizW<Npatcfg3Spec> {
        AtthizW::new(self, 24)
    }
}
#[doc = "NAND flash/PC card attribute space timing configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`npatcfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`npatcfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Npatcfg3Spec;
impl crate::RegisterSpec for Npatcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npatcfg3::R`](R) reader structure"]
impl crate::Readable for Npatcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`npatcfg3::W`](W) writer structure"]
impl crate::Writable for Npatcfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NPATCFG3 to value 0xfcfc_fcfc"]
impl crate::Resettable for Npatcfg3Spec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
