#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    #[doc = "0x02 - Performance Level Configuration"]
    pub plcfg: PLCFG,
    #[doc = "0x03 - Power Configuration"]
    pub pwcfg: PWCFG,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved1: [u8; 1usize],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
}
#[doc = "Sleep Configuration"]
pub struct SLEEPCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "Performance Level Configuration"]
pub struct PLCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Performance Level Configuration"]
pub mod plcfg;
#[doc = "Power Configuration"]
pub struct PWCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Configuration"]
pub mod pwcfg;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Standby Configuration"]
pub struct STDBYCFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Standby Configuration"]
pub mod stdbycfg;
