#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved0: [u8; 3usize],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved1: [u8; 6usize],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    _reserved2: [u8; 20usize],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    _reserved3: [u8; 20usize],
    #[doc = "0x54 - Peripheral non-secure status - Bridge A"]
    pub nonseca: NONSECA,
    #[doc = "0x58 - Peripheral non-secure status - Bridge B"]
    pub nonsecb: NONSECB,
    #[doc = "0x5c - Peripheral non-secure status - Bridge C"]
    pub nonsecc: NONSECC,
    _reserved4: [u8; 20usize],
    #[doc = "0x74 - Peripheral secure status locked - Bridge A"]
    pub seclocka: SECLOCKA,
    #[doc = "0x78 - Peripheral secure status locked - Bridge B"]
    pub seclockb: SECLOCKB,
    #[doc = "0x7c - Peripheral secure status locked - Bridge C"]
    pub seclockc: SECLOCKC,
}
#[doc = "Write control"]
pub struct WRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "Event control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event control"]
pub mod evctrl;
#[doc = "Interrupt enable clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt enable set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "Bridge interrupt flag status"]
pub struct INTFLAGAHB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub struct INTFLAGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub struct INTFLAGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub struct INTFLAGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "Peripheral write protection status - Bridge A"]
pub struct STATUSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "Peripheral write protection status - Bridge B"]
pub struct STATUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "Peripheral write protection status - Bridge C"]
pub struct STATUSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "Peripheral non-secure status - Bridge A"]
pub struct NONSECA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral non-secure status - Bridge A"]
pub mod nonseca;
#[doc = "Peripheral non-secure status - Bridge B"]
pub struct NONSECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral non-secure status - Bridge B"]
pub mod nonsecb;
#[doc = "Peripheral non-secure status - Bridge C"]
pub struct NONSECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral non-secure status - Bridge C"]
pub mod nonsecc;
#[doc = "Peripheral secure status locked - Bridge A"]
pub struct SECLOCKA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral secure status locked - Bridge A"]
pub mod seclocka;
#[doc = "Peripheral secure status locked - Bridge B"]
pub struct SECLOCKB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral secure status locked - Bridge B"]
pub mod seclockb;
#[doc = "Peripheral secure status locked - Bridge C"]
pub struct SECLOCKC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral secure status locked - Bridge C"]
pub mod seclockc;
