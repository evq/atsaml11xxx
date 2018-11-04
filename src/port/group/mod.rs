#[doc = "Data Direction"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction"]
pub mod dir;
#[doc = "Data Direction Clear"]
pub struct DIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "Data Direction Set"]
pub struct DIRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "Data Direction Toggle"]
pub struct DIRTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "Data Output Value"]
pub struct OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value"]
pub mod out;
#[doc = "Data Output Value Clear"]
pub struct OUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "Data Output Value Set"]
pub struct OUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "Data Output Value Toggle"]
pub struct OUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "Data Input Value"]
pub struct IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Write Configuration"]
pub struct WRCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "Event Input Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Input Control"]
pub mod evctrl;
#[doc = "Peripheral Multiplexing"]
pub struct PMUX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Multiplexing"]
pub mod pmux;
#[doc = "Pin Configuration"]
pub struct PINCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Pin Configuration"]
pub mod pincfg;
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
#[doc = "Security Attribution"]
pub struct NONSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution"]
pub mod nonsec;
#[doc = "Security Attribution Check"]
pub struct NSCHK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Check"]
pub mod nschk;
