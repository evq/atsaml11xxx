#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved1: [u8; 7usize],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved2: [u8; 2usize],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20 - Channel"]
    pub channel: [CHANNEL; 8],
    _reserved3: [u8; 192usize],
    #[doc = "0x120 - User Multiplexer n"]
    pub user: [USER; 23],
    _reserved4: [u8; 157usize],
    #[doc = "0x1d4 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x1d5 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1d6 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 1usize],
    #[doc = "0x1d8 - Channels Security Attribution"]
    pub nonsecchan: NONSECCHAN,
    #[doc = "0x1dc - Non-Secure Channels Check"]
    pub nschkchan: NSCHKCHAN,
    #[doc = "0x1e0 - Users Security Attribution"]
    pub nonsecuser: [NONSECUSER; 1],
    _reserved6: [u8; 12usize],
    #[doc = "0x1f0 - Non-Secure Users Check"]
    pub nschkuser: [NSCHKUSER; 1],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channel: self::channel::CHANNEL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r" Register block"]
#[doc = "Channel"]
pub mod channel;
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Software Event"]
pub struct SWEVT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Event"]
pub mod swevt;
#[doc = "Priority Control"]
pub struct PRICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "Channel Pending Interrupt"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "Interrupt Status"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels"]
pub struct BUSYCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Ready Users"]
pub struct READYUSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "User Multiplexer n"]
pub struct USER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "User Multiplexer n"]
pub mod user;
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
#[doc = "Channels Security Attribution"]
pub struct NONSECCHAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channels Security Attribution"]
pub mod nonsecchan;
#[doc = "Non-Secure Channels Check"]
pub struct NSCHKCHAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Secure Channels Check"]
pub mod nschkchan;
#[doc = "Users Security Attribution"]
pub struct NONSECUSER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Users Security Attribution"]
pub mod nonsecuser;
#[doc = "Non-Secure Users Check"]
pub struct NSCHKUSER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Secure Users Check"]
pub mod nschkuser;
