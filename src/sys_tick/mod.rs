#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick Control and Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - SysTick Reload Value Register"]
    pub rvr: RVR,
    #[doc = "0x08 - SysTick Current Value Register"]
    pub cvr: CVR,
    #[doc = "0x0c - SysTick Calibration Value Register"]
    pub calib: CALIB,
}
#[doc = "SysTick Control and Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Control and Status Register"]
pub mod csr;
#[doc = "SysTick Reload Value Register"]
pub struct RVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Reload Value Register"]
pub mod rvr;
#[doc = "SysTick Current Value Register"]
pub struct CVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Current Value Register"]
pub mod cvr;
#[doc = "SysTick Calibration Value Register"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Calibration Value Register"]
pub mod calib;
