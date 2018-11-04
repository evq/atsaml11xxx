#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "Reset Cause"]
pub struct RCAUSE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reset Cause"]
pub mod rcause;
