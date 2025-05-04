#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<Apb1enSpec>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<Apb1enSpec>;
#[doc = "TIMER1 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1en {
    #[doc = "0: Disable the selected module clock."]
    Disable = 0,
    #[doc = "1: Enable the selected module clock."]
    Enable = 1,
}
impl From<Timer1en> for bool {
    #[inline(always)]
    fn from(variant: Timer1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1EN` reader - TIMER1 timer clock enable"]
pub type Timer1enR = crate::BitReader<Timer1en>;
impl Timer1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1en {
        match self.bits {
            false => Timer1en::Disable,
            true => Timer1en::Enable,
        }
    }
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer1en::Disable
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer1en::Enable
    }
}
#[doc = "Field `TIMER1EN` writer - TIMER1 timer clock enable"]
pub type Timer1enW<'a, REG> = crate::BitWriter<'a, REG, Timer1en>;
impl<'a, REG> Timer1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1en::Disable)
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1en::Enable)
    }
}
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub use Timer1enR as Timer2enR;
#[doc = "Field `TIMER3EN` reader - TIMER3 timer clock enable"]
pub use Timer1enR as Timer3enR;
#[doc = "Field `TIMER4EN` reader - TIMER4 timer clock enable"]
pub use Timer1enR as Timer4enR;
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub use Timer1enR as Timer5enR;
#[doc = "Field `TIMER6EN` reader - TIMER6 timer clock enable"]
pub use Timer1enR as Timer6enR;
#[doc = "Field `TIMER11EN` reader - TIMER11 timer clock enable"]
pub use Timer1enR as Timer11enR;
#[doc = "Field `TIMER12EN` reader - TIMER12 timer clock enable"]
pub use Timer1enR as Timer12enR;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub use Timer1enR as Timer13enR;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub use Timer1enR as WwdgtenR;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use Timer1enR as Spi1enR;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use Timer1enR as Spi2enR;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use Timer1enR as Usart1enR;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub use Timer1enR as Usart2enR;
#[doc = "Field `UART3EN` reader - UART3 clock enable"]
pub use Timer1enR as Uart3enR;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub use Timer1enR as Uart4enR;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub use Timer1enR as I2c0enR;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use Timer1enR as I2c1enR;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub use Timer1enR as I2c2enR;
#[doc = "Field `CAN0EN` reader - CAN0 clock enable"]
pub use Timer1enR as Can0enR;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub use Timer1enR as Can1enR;
#[doc = "Field `PMUEN` reader - Power control clock enable"]
pub use Timer1enR as PmuenR;
#[doc = "Field `DACEN` reader - DAC clock enable"]
pub use Timer1enR as DacenR;
#[doc = "Field `UART6EN` reader - UART6 clock enable"]
pub use Timer1enR as Uart6enR;
#[doc = "Field `UART7EN` reader - UART7 clock enable"]
pub use Timer1enR as Uart7enR;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub use Timer1enW as Timer2enW;
#[doc = "Field `TIMER3EN` writer - TIMER3 timer clock enable"]
pub use Timer1enW as Timer3enW;
#[doc = "Field `TIMER4EN` writer - TIMER4 timer clock enable"]
pub use Timer1enW as Timer4enW;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub use Timer1enW as Timer5enW;
#[doc = "Field `TIMER6EN` writer - TIMER6 timer clock enable"]
pub use Timer1enW as Timer6enW;
#[doc = "Field `TIMER11EN` writer - TIMER11 timer clock enable"]
pub use Timer1enW as Timer11enW;
#[doc = "Field `TIMER12EN` writer - TIMER12 timer clock enable"]
pub use Timer1enW as Timer12enW;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub use Timer1enW as Timer13enW;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub use Timer1enW as WwdgtenW;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use Timer1enW as Spi1enW;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use Timer1enW as Spi2enW;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use Timer1enW as Usart1enW;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub use Timer1enW as Usart2enW;
#[doc = "Field `UART3EN` writer - UART3 clock enable"]
pub use Timer1enW as Uart3enW;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub use Timer1enW as Uart4enW;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub use Timer1enW as I2c0enW;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use Timer1enW as I2c1enW;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub use Timer1enW as I2c2enW;
#[doc = "Field `CAN0EN` writer - CAN0 clock enable"]
pub use Timer1enW as Can0enW;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub use Timer1enW as Can1enW;
#[doc = "Field `PMUEN` writer - Power control clock enable"]
pub use Timer1enW as PmuenW;
#[doc = "Field `DACEN` writer - DAC clock enable"]
pub use Timer1enW as DacenW;
#[doc = "Field `UART6EN` writer - UART6 clock enable"]
pub use Timer1enW as Uart6enW;
#[doc = "Field `UART7EN` writer - UART7 clock enable"]
pub use Timer1enW as Uart7enW;
impl R {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    pub fn timer1en(&self) -> Timer1enR {
        Timer1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> Timer2enR {
        Timer2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    pub fn timer3en(&self) -> Timer3enR {
        Timer3enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    pub fn timer4en(&self) -> Timer4enR {
        Timer4enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> Timer5enR {
        Timer5enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    pub fn timer6en(&self) -> Timer6enR {
        Timer6enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable"]
    #[inline(always)]
    pub fn timer11en(&self) -> Timer11enR {
        Timer11enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable"]
    #[inline(always)]
    pub fn timer12en(&self) -> Timer12enR {
        Timer12enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> Timer13enR {
        Timer13enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WwdgtenR {
        WwdgtenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3en(&self) -> Uart3enR {
        Uart3enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2c0enR {
        I2c0enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    pub fn can0en(&self) -> Can0enR {
        Can0enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> Can1enR {
        Can1enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PmuenR {
        PmuenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6en(&self) -> Uart6enR {
        Uart6enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&self) -> Uart7enR {
        Uart7enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    pub fn timer1en(&mut self) -> Timer1enW<Apb1enSpec> {
        Timer1enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&mut self) -> Timer2enW<Apb1enSpec> {
        Timer2enW::new(self, 1)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    pub fn timer3en(&mut self) -> Timer3enW<Apb1enSpec> {
        Timer3enW::new(self, 2)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    pub fn timer4en(&mut self) -> Timer4enW<Apb1enSpec> {
        Timer4enW::new(self, 3)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&mut self) -> Timer5enW<Apb1enSpec> {
        Timer5enW::new(self, 4)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    pub fn timer6en(&mut self) -> Timer6enW<Apb1enSpec> {
        Timer6enW::new(self, 5)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable"]
    #[inline(always)]
    pub fn timer11en(&mut self) -> Timer11enW<Apb1enSpec> {
        Timer11enW::new(self, 6)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable"]
    #[inline(always)]
    pub fn timer12en(&mut self) -> Timer12enW<Apb1enSpec> {
        Timer12enW::new(self, 7)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&mut self) -> Timer13enW<Apb1enSpec> {
        Timer13enW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&mut self) -> WwdgtenW<Apb1enSpec> {
        WwdgtenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<Apb1enSpec> {
        Spi1enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<Apb1enSpec> {
        Spi2enW::new(self, 15)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<Apb1enSpec> {
        Usart1enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<Apb1enSpec> {
        Usart2enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3en(&mut self) -> Uart3enW<Apb1enSpec> {
        Uart3enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> Uart4enW<Apb1enSpec> {
        Uart4enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2c0enW<Apb1enSpec> {
        I2c0enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<Apb1enSpec> {
        I2c1enW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<Apb1enSpec> {
        I2c2enW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    pub fn can0en(&mut self) -> Can0enW<Apb1enSpec> {
        Can0enW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&mut self) -> Can1enW<Apb1enSpec> {
        Can1enW::new(self, 26)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    pub fn pmuen(&mut self) -> PmuenW<Apb1enSpec> {
        PmuenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<Apb1enSpec> {
        DacenW::new(self, 29)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6en(&mut self) -> Uart6enW<Apb1enSpec> {
        Uart6enW::new(self, 30)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&mut self) -> Uart7enW<Apb1enSpec> {
        Uart7enW::new(self, 31)
    }
}
#[doc = "APB1 clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enSpec;
impl crate::RegisterSpec for Apb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for Apb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for Apb1enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for Apb1enSpec {}
