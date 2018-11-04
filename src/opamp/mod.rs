#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 1usize],
    #[doc = "0x02 - Status"]
    pub status: STATUS,
    _reserved1: [u8; 1usize],
    #[doc = "0x04 - OPAMP n Control"]
    pub opampctrl: [OPAMPCTRL; 3],
    #[doc = "0x10 - Resister Control"]
    pub resctrl: RESCTRL,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "OPAMP n Control"]
pub struct OPAMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP n Control"]
pub mod opampctrl;
#[doc = "Resister Control"]
pub struct RESCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Resister Control"]
pub mod resctrl;
