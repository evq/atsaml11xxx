#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x03 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Sequence Status"]
    pub seqstatus: SEQSTATUS,
    #[doc = "0x08 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x0a - Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x0c - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x0d - Sample Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x0e - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    #[doc = "0x10 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    #[doc = "0x12 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x14 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    _reserved0: [u8; 2usize],
    #[doc = "0x18 - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved1: [u8; 3usize],
    #[doc = "0x1c - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved2: [u8; 3usize],
    #[doc = "0x20 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved3: [u8; 2usize],
    #[doc = "0x24 - Result"]
    pub result: RESULT,
    _reserved4: [u8; 2usize],
    #[doc = "0x28 - Sequence Control"]
    pub seqctrl: SEQCTRL,
    #[doc = "0x2c - Calibration"]
    pub calib: CALIB,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Reference Control"]
pub struct REFCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reference Control"]
pub mod refctrl;
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
#[doc = "Sequence Status"]
pub struct SEQSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sequence Status"]
pub mod seqstatus;
#[doc = "Input Control"]
pub struct INPUTCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Average Control"]
pub struct AVGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "Sample Time Control"]
pub struct SAMPCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sample Time Control"]
pub mod sampctrl;
#[doc = "Window Monitor Lower Threshold"]
pub struct WINLT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "Window Monitor Upper Threshold"]
pub struct WINUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "Gain Correction"]
pub struct GAINCORR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "Offset Correction"]
pub struct OFFSETCORR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "Software Trigger"]
pub struct SWTRIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Result"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Result"]
pub mod result;
#[doc = "Sequence Control"]
pub struct SEQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "Calibration"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Calibration"]
pub mod calib;
