#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - SECCTRL"]
    pub secctrl: SECCTRL,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - SCFGB"]
    pub scfgb: SCFGB,
    #[doc = "0x08 - SCFGA"]
    pub scfga: SCFGA,
    #[doc = "0x0c - SCFGR"]
    pub scfgr: SCFGR,
}
#[doc = "SECCTRL"]
pub struct SECCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SECCTRL"]
pub mod secctrl;
#[doc = "SCFGB"]
pub struct SCFGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCFGB"]
pub mod scfgb;
#[doc = "SCFGA"]
pub struct SCFGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCFGA"]
pub mod scfga;
#[doc = "SCFGR"]
pub struct SCFGR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SCFGR"]
pub mod scfgr;
