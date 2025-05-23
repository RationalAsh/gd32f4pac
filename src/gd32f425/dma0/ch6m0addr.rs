#[doc = "Register `CH6M0ADDR` reader"]
pub type R = crate::R<Ch6m0addrSpec>;
#[doc = "Register `CH6M0ADDR` writer"]
pub type W = crate::W<Ch6m0addrSpec>;
#[doc = "Field `M0ADDR` reader - Memory 0 base address"]
pub type M0addrR = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 base address"]
pub type M0addrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 base address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0addrR {
        M0addrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 base address"]
    #[inline(always)]
    pub fn m0addr(&mut self) -> M0addrW<Ch6m0addrSpec> {
        M0addrW::new(self, 0)
    }
}
#[doc = "Channel 6 memory 0 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6m0addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6m0addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6m0addrSpec;
impl crate::RegisterSpec for Ch6m0addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6m0addr::R`](R) reader structure"]
impl crate::Readable for Ch6m0addrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6m0addr::W`](W) writer structure"]
impl crate::Writable for Ch6m0addrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6M0ADDR to value 0"]
impl crate::Resettable for Ch6m0addrSpec {}
