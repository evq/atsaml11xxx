#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: XOSCCTRL,
    #[doc = "0x16 - Clock Failure Detector Prescaler"]
    pub cfdpresc: CFDPRESC,
    _reserved1: [u8; 1usize],
    #[doc = "0x18 - 16MHz Internal Oscillator (OSC16M) Control"]
    pub osc16mctrl: OSC16MCTRL,
    _reserved2: [u8; 3usize],
    #[doc = "0x1c - DFLLULP Control"]
    pub dfllulpctrl: DFLLULPCTRL,
    #[doc = "0x1e - DFLLULP Dither Control"]
    pub dfllulpdither: DFLLULPDITHER,
    #[doc = "0x1f - DFLLULP Read Request"]
    pub dfllulprreq: DFLLULPRREQ,
    #[doc = "0x20 - DFLLULP Delay Value"]
    pub dfllulpdly: DFLLULPDLY,
    #[doc = "0x24 - DFLLULP Target Ratio"]
    pub dfllulpratio: DFLLULPRATIO,
    #[doc = "0x28 - DFLLULP Synchronization Busy"]
    pub dfllulpsyncbusy: DFLLULPSYNCBUSY,
    #[doc = "0x2c - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved3: [u8; 3usize],
    #[doc = "0x30 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x34 - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x38 - DPLL Prescaler"]
    pub dpllpresc: DPLLPRESC,
    _reserved4: [u8; 3usize],
    #[doc = "0x3c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    _reserved5: [u8; 3usize],
    #[doc = "0x40 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub struct XOSCCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "Clock Failure Detector Prescaler"]
pub struct CFDPRESC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub struct OSC16MCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub mod osc16mctrl;
#[doc = "DFLLULP Control"]
pub struct DFLLULPCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DFLLULP Control"]
pub mod dfllulpctrl;
#[doc = "DFLLULP Dither Control"]
pub struct DFLLULPDITHER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DFLLULP Dither Control"]
pub mod dfllulpdither;
#[doc = "DFLLULP Read Request"]
pub struct DFLLULPRREQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DFLLULP Read Request"]
pub mod dfllulprreq;
#[doc = "DFLLULP Delay Value"]
pub struct DFLLULPDLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLLULP Delay Value"]
pub mod dfllulpdly;
#[doc = "DFLLULP Target Ratio"]
pub struct DFLLULPRATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLLULP Target Ratio"]
pub mod dfllulpratio;
#[doc = "DFLLULP Synchronization Busy"]
pub struct DFLLULPSYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLLULP Synchronization Busy"]
pub mod dfllulpsyncbusy;
#[doc = "DPLL Control A"]
pub struct DPLLCTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control"]
pub struct DPLLRATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B"]
pub struct DPLLCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Prescaler"]
pub struct DPLLPRESC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLL Synchronization Busy"]
pub struct DPLLSYNCBUSY {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status"]
pub struct DPLLSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Status"]
pub mod dpllstatus;
