#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    omode: Omode,
    ospd: Ospd,
    pud: Pud,
    istat: Istat,
    octl: Octl,
    bop: Bop,
    lock: Lock,
    afsel0: Afsel0,
    afsel1: Afsel1,
    bc: Bc,
    tg: Tg,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - GPIO port output mode register"]
    #[inline(always)]
    pub const fn omode(&self) -> &Omode {
        &self.omode
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn ospd(&self) -> &Ospd {
        &self.ospd
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn pud(&self) -> &Pud {
        &self.pud
    }
    #[doc = "0x10 - GPIO port input status register"]
    #[inline(always)]
    pub const fn istat(&self) -> &Istat {
        &self.istat
    }
    #[doc = "0x14 - GPIO port output control register"]
    #[inline(always)]
    pub const fn octl(&self) -> &Octl {
        &self.octl
    }
    #[doc = "0x18 - GPIO port bit operate register"]
    #[inline(always)]
    pub const fn bop(&self) -> &Bop {
        &self.bop
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x20 - GPIO alternate function selected register 0"]
    #[inline(always)]
    pub const fn afsel0(&self) -> &Afsel0 {
        &self.afsel0
    }
    #[doc = "0x24 - GPIO alternate function selected register 1"]
    #[inline(always)]
    pub const fn afsel1(&self) -> &Afsel1 {
        &self.afsel1
    }
    #[doc = "0x28 - Bit clear register"]
    #[inline(always)]
    pub const fn bc(&self) -> &Bc {
        &self.bc
    }
    #[doc = "0x2c - Port bit toggle register"]
    #[inline(always)]
    pub const fn tg(&self) -> &Tg {
        &self.tg
    }
}
#[doc = "CTL (rw) register accessor: GPIO port control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "GPIO port control register"]
pub mod ctl;
#[doc = "OMODE (rw) register accessor: GPIO port output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`omode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omode`] module"]
#[doc(alias = "OMODE")]
pub type Omode = crate::Reg<omode::OmodeSpec>;
#[doc = "GPIO port output mode register"]
pub mod omode;
#[doc = "OSPD (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospd`] module"]
#[doc(alias = "OSPD")]
pub type Ospd = crate::Reg<ospd::OspdSpec>;
#[doc = "GPIO port output speed register"]
pub mod ospd;
#[doc = "PUD (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pud`] module"]
#[doc(alias = "PUD")]
pub type Pud = crate::Reg<pud::PudSpec>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pud;
#[doc = "ISTAT (r) register accessor: GPIO port input status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`] module"]
#[doc(alias = "ISTAT")]
pub type Istat = crate::Reg<istat::IstatSpec>;
#[doc = "GPIO port input status register"]
pub mod istat;
#[doc = "OCTL (rw) register accessor: GPIO port output control register\n\nYou can [`read`](crate::Reg::read) this register and get [`octl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl`] module"]
#[doc(alias = "OCTL")]
pub type Octl = crate::Reg<octl::OctlSpec>;
#[doc = "GPIO port output control register"]
pub mod octl;
#[doc = "BOP (w) register accessor: GPIO port bit operate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bop`] module"]
#[doc(alias = "BOP")]
pub type Bop = crate::Reg<bop::BopSpec>;
#[doc = "GPIO port bit operate register"]
pub mod bop;
#[doc = "LOCK (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "GPIO port configuration lock register"]
pub mod lock;
#[doc = "AFSEL0 (rw) register accessor: GPIO alternate function selected register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`afsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsel0`] module"]
#[doc(alias = "AFSEL0")]
pub type Afsel0 = crate::Reg<afsel0::Afsel0Spec>;
#[doc = "GPIO alternate function selected register 0"]
pub mod afsel0;
#[doc = "AFSEL1 (rw) register accessor: GPIO alternate function selected register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`afsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsel1`] module"]
#[doc(alias = "AFSEL1")]
pub type Afsel1 = crate::Reg<afsel1::Afsel1Spec>;
#[doc = "GPIO alternate function selected register 1"]
pub mod afsel1;
#[doc = "BC (w) register accessor: Bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bc`] module"]
#[doc(alias = "BC")]
pub type Bc = crate::Reg<bc::BcSpec>;
#[doc = "Bit clear register"]
pub mod bc;
#[doc = "TG (w) register accessor: Port bit toggle register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg`] module"]
#[doc(alias = "TG")]
pub type Tg = crate::Reg<tg::TgSpec>;
#[doc = "Port bit toggle register"]
pub mod tg;
