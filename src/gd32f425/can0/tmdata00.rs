#[doc = "Register `TMDATA00` reader"]
pub type R = crate::R<Tmdata00Spec>;
#[doc = "Register `TMDATA00` writer"]
pub type W = crate::W<Tmdata00Spec>;
#[doc = "Field `DB0` reader - Data byte 0"]
pub type Db0R = crate::FieldReader;
#[doc = "Field `DB0` writer - Data byte 0"]
pub type Db0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB1` reader - Data byte 1"]
pub type Db1R = crate::FieldReader;
#[doc = "Field `DB1` writer - Data byte 1"]
pub type Db1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB2` reader - Data byte 2"]
pub type Db2R = crate::FieldReader;
#[doc = "Field `DB2` writer - Data byte 2"]
pub type Db2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB3` reader - Data byte 3"]
pub type Db3R = crate::FieldReader;
#[doc = "Field `DB3` writer - Data byte 3"]
pub type Db3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&self) -> Db0R {
        Db0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&self) -> Db1R {
        Db1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&self) -> Db2R {
        Db2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&self) -> Db3R {
        Db3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&mut self) -> Db0W<Tmdata00Spec> {
        Db0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&mut self) -> Db1W<Tmdata00Spec> {
        Db1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&mut self) -> Db2W<Tmdata00Spec> {
        Db2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&mut self) -> Db3W<Tmdata00Spec> {
        Db3W::new(self, 24)
    }
}
#[doc = "Transmit mailbox data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdata00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdata00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmdata00Spec;
impl crate::RegisterSpec for Tmdata00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdata00::R`](R) reader structure"]
impl crate::Readable for Tmdata00Spec {}
#[doc = "`write(|w| ..)` method takes [`tmdata00::W`](W) writer structure"]
impl crate::Writable for Tmdata00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMDATA00 to value 0"]
impl crate::Resettable for Tmdata00Spec {}
