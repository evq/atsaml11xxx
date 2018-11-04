#[doc = "Channel n Control"]
pub struct CHANNEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Control"]
pub mod channel;
#[doc = "Channel n Interrupt Enable Clear"]
pub struct CHINTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "Channel n Interrupt Enable Set"]
pub struct CHINTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub struct CHINTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "Channel n Status"]
pub struct CHSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Status"]
pub mod chstatus;
