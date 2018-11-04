#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Control C"]
    pub ctrlc: CTRLC,
    _reserved1: [u8; 1usize],
    #[doc = "0x0a - Event Control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 1usize],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 3usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 3usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 3usize],
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Address"]
    pub addr: ADDR,
    #[doc = "0x20 - Secure Unlock Register"]
    pub sulck: SULCK,
    #[doc = "0x22 - Non-Secure Unlock Register"]
    pub nsulck: NSULCK,
    #[doc = "0x24 - NVM Parameter"]
    pub param: PARAM,
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - Data Scramble Configuration"]
    pub dscc: DSCC,
    #[doc = "0x34 - Security Control"]
    pub secctrl: SECCTRL,
    #[doc = "0x38 - Secure Boot Configuration"]
    pub scfgb: SCFGB,
    #[doc = "0x3c - Secure Application and Data Configuration"]
    pub scfgad: SCFGAD,
    #[doc = "0x40 - Non-secure Write Enable"]
    pub nonsec: NONSEC,
    #[doc = "0x44 - Non-secure Write Reference Value"]
    pub nschk: NSCHK,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address"]
pub mod addr;
#[doc = "Secure Unlock Register"]
pub struct SULCK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Secure Unlock Register"]
pub mod sulck;
#[doc = "Non-Secure Unlock Register"]
pub struct NSULCK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Non-Secure Unlock Register"]
pub mod nsulck;
#[doc = "NVM Parameter"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "Data Scramble Configuration"]
pub struct DSCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Scramble Configuration"]
pub mod dscc;
#[doc = "Security Control"]
pub struct SECCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Control"]
pub mod secctrl;
#[doc = "Secure Boot Configuration"]
pub struct SCFGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Boot Configuration"]
pub mod scfgb;
#[doc = "Secure Application and Data Configuration"]
pub struct SCFGAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Application and Data Configuration"]
pub mod scfgad;
#[doc = "Non-secure Write Enable"]
pub struct NONSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-secure Write Enable"]
pub mod nonsec;
#[doc = "Non-secure Write Reference Value"]
pub struct NSCHK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-secure Write Reference Value"]
pub mod nschk;
