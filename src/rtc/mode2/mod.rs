#[doc = "MODE2 Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "MODE2 Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Control B"]
pub mod ctrlb;
#[doc = "MODE2 Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "MODE2 Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE2 Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "MODE2 Synchronization Busy Status"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "Frequency Correction"]
pub struct FREQCORR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "MODE2 Clock Value"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "General Purpose"]
pub struct GP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose"]
pub mod gp;
#[doc = "Tamper Control"]
pub struct TAMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "MODE2 Timestamp"]
pub struct TIMESTAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Timestamp"]
pub mod timestamp;
#[doc = "Tamper ID"]
pub struct TAMPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "Tamper Control B"]
pub struct TAMPCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tamper Control B"]
pub mod tampctrlb;
#[doc = r" Register block"]
#[repr(C)]
pub struct MODE2_ALARM {
    #[doc = "0x00 - MODE2_ALARM Alarm n Value"]
    pub alarm: self::mode2_alarm::ALARM,
    #[doc = "0x04 - MODE2_ALARM Alarm n Mask"]
    pub mask: self::mode2_alarm::MASK,
}
#[doc = r" Register block"]
#[doc = "MODE2 Alarm"]
pub mod mode2_alarm;
