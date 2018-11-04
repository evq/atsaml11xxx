#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - BOD33 Control"]
    pub bod33: BOD33,
    #[doc = "0x14 - BOD12 Control"]
    pub bod12: BOD12,
    #[doc = "0x18 - VREG Control"]
    pub vreg: VREG,
    #[doc = "0x1c - VREF Control"]
    pub vref: VREF,
    _reserved0: [u8; 12usize],
    #[doc = "0x2c - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x30 - VREG Suspend Control"]
    pub vregsusp: VREGSUSP,
}
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Power and Clocks Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "BOD33 Control"]
pub struct BOD33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "BOD12 Control"]
pub struct BOD12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD12 Control"]
pub mod bod12;
#[doc = "VREG Control"]
pub struct VREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF Control"]
pub struct VREF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREF Control"]
pub mod vref;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "VREG Suspend Control"]
pub struct VREGSUSP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREG Suspend Control"]
pub mod vregsusp;
