#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Synchronization Busy Status"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Data Scramble Control"]
    pub dscc: DSCC,
    #[doc = "0x10 - Permutation Write"]
    pub permw: PERMW,
    #[doc = "0x11 - Permutation Read"]
    pub permr: PERMR,
    _reserved1: [u8; 238usize],
    #[doc = "0x100 - TrustRAM"]
    pub ram: [RAM; 64],
}
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
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
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Synchronization Busy Status"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "Data Scramble Control"]
pub struct DSCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Scramble Control"]
pub mod dscc;
#[doc = "Permutation Write"]
pub struct PERMW {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Permutation Write"]
pub mod permw;
#[doc = "Permutation Read"]
pub struct PERMR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Permutation Read"]
pub mod permr;
#[doc = "TrustRAM"]
pub struct RAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TrustRAM"]
pub mod ram;
