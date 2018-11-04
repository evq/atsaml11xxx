#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 32-bit Counter with Single 32-bit Compare"]
    pub mode0: MODE0,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct MODE0 {
    #[doc = "0x00 - MODE0 Control A"]
    pub ctrla: self::mode0::CTRLA,
    #[doc = "0x02 - MODE0 Control B"]
    pub ctrlb: self::mode0::CTRLB,
    #[doc = "0x04 - MODE0 Event Control"]
    pub evctrl: self::mode0::EVCTRL,
    #[doc = "0x08 - MODE0 Interrupt Enable Clear"]
    pub intenclr: self::mode0::INTENCLR,
    #[doc = "0x0a - MODE0 Interrupt Enable Set"]
    pub intenset: self::mode0::INTENSET,
    #[doc = "0x0c - MODE0 Interrupt Flag Status and Clear"]
    pub intflag: self::mode0::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode0::DBGCTRL,
    _reserved0: [u8; 1usize],
    #[doc = "0x10 - MODE0 Synchronization Busy Status"]
    pub syncbusy: self::mode0::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode0::FREQCORR,
    _reserved1: [u8; 3usize],
    #[doc = "0x18 - MODE0 Counter Value"]
    pub count: self::mode0::COUNT,
    _reserved2: [u8; 4usize],
    #[doc = "0x20 - MODE0 Compare n Value"]
    pub comp: [self::mode0::COMP; 1],
    _reserved3: [u8; 28usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode0::GP; 2],
    _reserved4: [u8; 24usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode0::TAMPCTRL,
    #[doc = "0x64 - MODE0 Timestamp"]
    pub timestamp: self::mode0::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode0::TAMPID,
    #[doc = "0x6c - Tamper Control B"]
    pub tampctrlb: self::mode0::TAMPCTRLB,
}
#[doc = r" Register block"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = r" Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control A"]
    pub ctrla: self::mode1::CTRLA,
    #[doc = "0x02 - MODE1 Control B"]
    pub ctrlb: self::mode1::CTRLB,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: self::mode1::EVCTRL,
    #[doc = "0x08 - MODE1 Interrupt Enable Clear"]
    pub intenclr: self::mode1::INTENCLR,
    #[doc = "0x0a - MODE1 Interrupt Enable Set"]
    pub intenset: self::mode1::INTENSET,
    #[doc = "0x0c - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: self::mode1::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode1::DBGCTRL,
    _reserved0: [u8; 1usize],
    #[doc = "0x10 - MODE1 Synchronization Busy Status"]
    pub syncbusy: self::mode1::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode1::FREQCORR,
    _reserved1: [u8; 3usize],
    #[doc = "0x18 - MODE1 Counter Value"]
    pub count: self::mode1::COUNT,
    _reserved2: [u8; 2usize],
    #[doc = "0x1c - MODE1 Counter Period"]
    pub per: self::mode1::PER,
    _reserved3: [u8; 2usize],
    #[doc = "0x20 - MODE1 Compare n Value"]
    pub comp: [self::mode1::COMP; 2],
    _reserved4: [u8; 28usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode1::GP; 2],
    _reserved5: [u8; 24usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode1::TAMPCTRL,
    #[doc = "0x64 - MODE1 Timestamp"]
    pub timestamp: self::mode1::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode1::TAMPID,
    #[doc = "0x6c - Tamper Control B"]
    pub tampctrlb: self::mode1::TAMPCTRLB,
}
#[doc = r" Register block"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = r" Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control A"]
    pub ctrla: self::mode2::CTRLA,
    #[doc = "0x02 - MODE2 Control B"]
    pub ctrlb: self::mode2::CTRLB,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: self::mode2::EVCTRL,
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    pub intenclr: self::mode2::INTENCLR,
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    pub intenset: self::mode2::INTENSET,
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: self::mode2::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode2::DBGCTRL,
    _reserved0: [u8; 1usize],
    #[doc = "0x10 - MODE2 Synchronization Busy Status"]
    pub syncbusy: self::mode2::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode2::FREQCORR,
    _reserved1: [u8; 3usize],
    #[doc = "0x18 - MODE2 Clock Value"]
    pub clock: self::mode2::CLOCK,
    _reserved2: [u8; 4usize],
    #[doc = "0x20 - MODE2 Alarm"]
    pub mode2_alarm0: self::mode2::MODE2_ALARM,
    _reserved3: [u8; 27usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode2::GP; 2],
    _reserved4: [u8; 24usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode2::TAMPCTRL,
    #[doc = "0x64 - MODE2 Timestamp"]
    pub timestamp: self::mode2::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode2::TAMPID,
    #[doc = "0x6c - Tamper Control B"]
    pub tampctrlb: self::mode2::TAMPCTRLB,
}
#[doc = r" Register block"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
