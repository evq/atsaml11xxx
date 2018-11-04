#![doc = "Peripheral access API for ATSAML11E16A microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT();
    fn RTC();
    fn EIC_0();
    fn EIC_1();
    fn EIC_2();
    fn EIC_3();
    fn EIC_OTHER();
    fn FREQM();
    fn NVMCTRL();
    fn PORT();
    fn DMAC_0();
    fn DMAC_1();
    fn DMAC_2();
    fn DMAC_3();
    fn DMAC_OTHER();
    fn EVSYS_0();
    fn EVSYS_1();
    fn EVSYS_2();
    fn EVSYS_3();
    fn EVSYS_NSCHK();
    fn PAC();
    fn SERCOM0_0();
    fn SERCOM0_1();
    fn SERCOM0_2();
    fn SERCOM0_OTHER();
    fn SERCOM1_0();
    fn SERCOM1_1();
    fn SERCOM1_2();
    fn SERCOM1_OTHER();
    fn SERCOM2_0();
    fn SERCOM2_1();
    fn SERCOM2_2();
    fn SERCOM2_OTHER();
    fn TC0();
    fn TC1();
    fn TC2();
    fn ADC_OTHER();
    fn ADC_RESRDY();
    fn AC();
    fn DAC_UNDERRUN_A();
    fn DAC_EMPTY();
    fn PTC();
    fn TRNG();
    fn TRAM();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: EIC_0 },
    Vector { _handler: EIC_1 },
    Vector { _handler: EIC_2 },
    Vector { _handler: EIC_3 },
    Vector {
        _handler: EIC_OTHER,
    },
    Vector { _handler: FREQM },
    Vector { _handler: NVMCTRL },
    Vector { _handler: PORT },
    Vector { _handler: DMAC_0 },
    Vector { _handler: DMAC_1 },
    Vector { _handler: DMAC_2 },
    Vector { _handler: DMAC_3 },
    Vector {
        _handler: DMAC_OTHER,
    },
    Vector { _handler: EVSYS_0 },
    Vector { _handler: EVSYS_1 },
    Vector { _handler: EVSYS_2 },
    Vector { _handler: EVSYS_3 },
    Vector {
        _handler: EVSYS_NSCHK,
    },
    Vector { _handler: PAC },
    Vector {
        _handler: SERCOM0_0,
    },
    Vector {
        _handler: SERCOM0_1,
    },
    Vector {
        _handler: SERCOM0_2,
    },
    Vector {
        _handler: SERCOM0_OTHER,
    },
    Vector {
        _handler: SERCOM1_0,
    },
    Vector {
        _handler: SERCOM1_1,
    },
    Vector {
        _handler: SERCOM1_2,
    },
    Vector {
        _handler: SERCOM1_OTHER,
    },
    Vector {
        _handler: SERCOM2_0,
    },
    Vector {
        _handler: SERCOM2_1,
    },
    Vector {
        _handler: SERCOM2_2,
    },
    Vector {
        _handler: SERCOM2_OTHER,
    },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector {
        _handler: ADC_OTHER,
    },
    Vector {
        _handler: ADC_RESRDY,
    },
    Vector { _handler: AC },
    Vector {
        _handler: DAC_UNDERRUN_A,
    },
    Vector {
        _handler: DAC_EMPTY,
    },
    Vector { _handler: PTC },
    Vector { _handler: TRNG },
    Vector { _handler: TRAM },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "1 - WDT"]
    WDT,
    #[doc = "2 - RTC"]
    RTC,
    #[doc = "3 - EIC_0"]
    EIC_0,
    #[doc = "4 - EIC_1"]
    EIC_1,
    #[doc = "5 - EIC_2"]
    EIC_2,
    #[doc = "6 - EIC_3"]
    EIC_3,
    #[doc = "7 - EIC_OTHER"]
    EIC_OTHER,
    #[doc = "8 - FREQM"]
    FREQM,
    #[doc = "9 - NVMCTRL"]
    NVMCTRL,
    #[doc = "10 - PORT"]
    PORT,
    #[doc = "11 - DMAC_0"]
    DMAC_0,
    #[doc = "12 - DMAC_1"]
    DMAC_1,
    #[doc = "13 - DMAC_2"]
    DMAC_2,
    #[doc = "14 - DMAC_3"]
    DMAC_3,
    #[doc = "15 - DMAC_OTHER"]
    DMAC_OTHER,
    #[doc = "16 - EVSYS_0"]
    EVSYS_0,
    #[doc = "17 - EVSYS_1"]
    EVSYS_1,
    #[doc = "18 - EVSYS_2"]
    EVSYS_2,
    #[doc = "19 - EVSYS_3"]
    EVSYS_3,
    #[doc = "20 - EVSYS_NSCHK"]
    EVSYS_NSCHK,
    #[doc = "21 - PAC"]
    PAC,
    #[doc = "22 - SERCOM0_0"]
    SERCOM0_0,
    #[doc = "23 - SERCOM0_1"]
    SERCOM0_1,
    #[doc = "24 - SERCOM0_2"]
    SERCOM0_2,
    #[doc = "25 - SERCOM0_OTHER"]
    SERCOM0_OTHER,
    #[doc = "26 - SERCOM1_0"]
    SERCOM1_0,
    #[doc = "27 - SERCOM1_1"]
    SERCOM1_1,
    #[doc = "28 - SERCOM1_2"]
    SERCOM1_2,
    #[doc = "29 - SERCOM1_OTHER"]
    SERCOM1_OTHER,
    #[doc = "30 - SERCOM2_0"]
    SERCOM2_0,
    #[doc = "31 - SERCOM2_1"]
    SERCOM2_1,
    #[doc = "32 - SERCOM2_2"]
    SERCOM2_2,
    #[doc = "33 - SERCOM2_OTHER"]
    SERCOM2_OTHER,
    #[doc = "34 - TC0"]
    TC0,
    #[doc = "35 - TC1"]
    TC1,
    #[doc = "36 - TC2"]
    TC2,
    #[doc = "37 - ADC_OTHER"]
    ADC_OTHER,
    #[doc = "38 - ADC_RESRDY"]
    ADC_RESRDY,
    #[doc = "39 - AC"]
    AC,
    #[doc = "40 - DAC_UNDERRUN_A"]
    DAC_UNDERRUN_A,
    #[doc = "41 - DAC_EMPTY"]
    DAC_EMPTY,
    #[doc = "42 - PTC"]
    PTC,
    #[doc = "43 - TRNG"]
    TRNG,
    #[doc = "44 - TRAM"]
    TRAM,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT => 1,
            Interrupt::RTC => 2,
            Interrupt::EIC_0 => 3,
            Interrupt::EIC_1 => 4,
            Interrupt::EIC_2 => 5,
            Interrupt::EIC_3 => 6,
            Interrupt::EIC_OTHER => 7,
            Interrupt::FREQM => 8,
            Interrupt::NVMCTRL => 9,
            Interrupt::PORT => 10,
            Interrupt::DMAC_0 => 11,
            Interrupt::DMAC_1 => 12,
            Interrupt::DMAC_2 => 13,
            Interrupt::DMAC_3 => 14,
            Interrupt::DMAC_OTHER => 15,
            Interrupt::EVSYS_0 => 16,
            Interrupt::EVSYS_1 => 17,
            Interrupt::EVSYS_2 => 18,
            Interrupt::EVSYS_3 => 19,
            Interrupt::EVSYS_NSCHK => 20,
            Interrupt::PAC => 21,
            Interrupt::SERCOM0_0 => 22,
            Interrupt::SERCOM0_1 => 23,
            Interrupt::SERCOM0_2 => 24,
            Interrupt::SERCOM0_OTHER => 25,
            Interrupt::SERCOM1_0 => 26,
            Interrupt::SERCOM1_1 => 27,
            Interrupt::SERCOM1_2 => 28,
            Interrupt::SERCOM1_OTHER => 29,
            Interrupt::SERCOM2_0 => 30,
            Interrupt::SERCOM2_1 => 31,
            Interrupt::SERCOM2_2 => 32,
            Interrupt::SERCOM2_OTHER => 33,
            Interrupt::TC0 => 34,
            Interrupt::TC1 => 35,
            Interrupt::TC2 => 36,
            Interrupt::ADC_OTHER => 37,
            Interrupt::ADC_RESRDY => 38,
            Interrupt::AC => 39,
            Interrupt::DAC_UNDERRUN_A => 40,
            Interrupt::DAC_EMPTY => 41,
            Interrupt::PTC => 42,
            Interrupt::TRNG => 43,
            Interrupt::TRAM => 44,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ac::RegisterBlock {
        1073755136 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    fn deref(&self) -> &ac::RegisterBlock {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1107303424 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog Digital Converter"]
pub mod adc;
#[doc = "Configurable Custom Logic"]
pub struct CCL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCL {}
impl CCL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccl::RegisterBlock {
        1107307520 as *const _
    }
}
impl Deref for CCL {
    type Target = ccl::RegisterBlock;
    fn deref(&self) -> &ccl::RegisterBlock {
        unsafe { &*CCL::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub mod ccl;
#[doc = "Digital Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1107304448 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital Analog Converter"]
pub mod dac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmac::RegisterBlock {
        1090543616 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &dmac::RegisterBlock {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsu::RegisterBlock {
        1090527232 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    fn deref(&self) -> &dsu::RegisterBlock {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eic::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &eic::RegisterBlock {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "EIC_SEC"]
pub struct EIC_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC_SEC {}
impl EIC_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eic::RegisterBlock {
        1073752576 as *const _
    }
}
impl Deref for EIC_SEC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &eic::RegisterBlock {
        unsafe { &*EIC_SEC::ptr() }
    }
}
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const evsys::RegisterBlock {
        1107296256 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &evsys::RegisterBlock {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "EVSYS_SEC"]
pub struct EVSYS_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS_SEC {}
impl EVSYS_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const evsys::RegisterBlock {
        1107296768 as *const _
    }
}
impl Deref for EVSYS_SEC {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &evsys::RegisterBlock {
        unsafe { &*EVSYS_SEC::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const freqm::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    fn deref(&self) -> &freqm::RegisterBlock {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gclk::RegisterBlock {
        1073748992 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    fn deref(&self) -> &gclk::RegisterBlock {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "Implementation Defined Attribution Unit"]
pub struct IDAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAU {}
impl IDAU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const idau::RegisterBlock {
        1090519040 as *const _
    }
}
impl Deref for IDAU {
    type Target = idau::RegisterBlock;
    fn deref(&self) -> &idau::RegisterBlock {
        unsafe { &*IDAU::ptr() }
    }
}
#[doc = "Implementation Defined Attribution Unit"]
pub mod idau;
#[doc = "Main Clock"]
pub struct MCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCLK {}
impl MCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mclk::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for MCLK {
    type Target = mclk::RegisterBlock;
    fn deref(&self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }
}
#[doc = "Main Clock"]
pub mod mclk;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmctrl::RegisterBlock {
        1090535424 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &nvmctrl::RegisterBlock {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "NVMCTRL_SEC"]
pub struct NVMCTRL_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL_SEC {}
impl NVMCTRL_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmctrl::RegisterBlock {
        1090539520 as *const _
    }
}
impl Deref for NVMCTRL_SEC {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &nvmctrl::RegisterBlock {
        unsafe { &*NVMCTRL_SEC::ptr() }
    }
}
#[doc = "Operational Amplifier"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const opamp::RegisterBlock {
        1107308544 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    fn deref(&self) -> &opamp::RegisterBlock {
        unsafe { &*OPAMP::ptr() }
    }
}
#[doc = "Operational Amplifier"]
pub mod opamp;
#[doc = "Oscillators Control"]
pub struct OSCCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCCTRL {}
impl OSCCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const oscctrl::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for OSCCTRL {
    type Target = oscctrl::RegisterBlock;
    fn deref(&self) -> &oscctrl::RegisterBlock {
        unsafe { &*OSCCTRL::ptr() }
    }
}
#[doc = "Oscillators Control"]
pub mod oscctrl;
#[doc = "32k Oscillators Control"]
pub struct OSC32KCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC32KCTRL {}
impl OSC32KCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const osc32kctrl::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for OSC32KCTRL {
    type Target = osc32kctrl::RegisterBlock;
    fn deref(&self) -> &osc32kctrl::RegisterBlock {
        unsafe { &*OSC32KCTRL::ptr() }
    }
}
#[doc = "32k Oscillators Control"]
pub mod osc32kctrl;
#[doc = "Peripheral Access Controller"]
pub struct PAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC {}
impl PAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pac::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for PAC {
    type Target = pac::RegisterBlock;
    fn deref(&self) -> &pac::RegisterBlock {
        unsafe { &*PAC::ptr() }
    }
}
#[doc = "Peripheral Access Controller"]
pub mod pac;
#[doc = "PAC_SEC"]
pub struct PAC_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC_SEC {}
impl PAC_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pac::RegisterBlock {
        1073742336 as *const _
    }
}
impl Deref for PAC_SEC {
    type Target = pac::RegisterBlock;
    fn deref(&self) -> &pac::RegisterBlock {
        unsafe { &*PAC_SEC::ptr() }
    }
}
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pm::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &pm::RegisterBlock {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "PORT_SEC"]
pub struct PORT_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_SEC {}
impl PORT_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1073754624 as *const _
    }
}
impl Deref for PORT_SEC {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT_SEC::ptr() }
    }
}
#[doc = "PORT_IOBUS"]
pub struct PORT_IOBUS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS {}
impl PORT_IOBUS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1610612736 as *const _
    }
}
impl Deref for PORT_IOBUS {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT_IOBUS::ptr() }
    }
}
#[doc = "PORT_IOBUS_SEC"]
pub struct PORT_IOBUS_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS_SEC {}
impl PORT_IOBUS_SEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1610613248 as *const _
    }
}
impl Deref for PORT_IOBUS_SEC {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT_IOBUS_SEC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rstc::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &rstc::RegisterBlock {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073751040 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Serial Communication Interface"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sercom0::RegisterBlock {
        1107297280 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &sercom0::RegisterBlock {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod sercom0;
#[doc = "SERCOM1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sercom0::RegisterBlock {
        1107298304 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &sercom0::RegisterBlock {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "SERCOM2"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sercom0::RegisterBlock {
        1107299328 as *const _
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &sercom0::RegisterBlock {
        unsafe { &*SERCOM2::ptr() }
    }
}
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const supc::RegisterBlock {
        1073747968 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &supc::RegisterBlock {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Basic Timer Counter"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1107300352 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub mod tc0;
#[doc = "TC1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1107301376 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "TC2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1107302400 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "TrustRAM"]
pub struct TRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRAM {}
impl TRAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tram::RegisterBlock {
        1107309568 as *const _
    }
}
impl Deref for TRAM {
    type Target = tram::RegisterBlock;
    fn deref(&self) -> &tram::RegisterBlock {
        unsafe { &*TRAM::ptr() }
    }
}
#[doc = "TrustRAM"]
pub mod tram;
#[doc = "True Random Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng::RegisterBlock {
        1107306496 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &trng::RegisterBlock {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Generator"]
pub mod trng;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys_tick::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &sys_tick::RegisterBlock {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "System Control Registers"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const system_control::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &system_control::RegisterBlock {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Registers"]
pub mod system_control;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CCL"]
    pub CCL: CCL,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EIC_SEC"]
    pub EIC_SEC: EIC_SEC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "EVSYS_SEC"]
    pub EVSYS_SEC: EVSYS_SEC,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "IDAU"]
    pub IDAU: IDAU,
    #[doc = "MCLK"]
    pub MCLK: MCLK,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "NVMCTRL_SEC"]
    pub NVMCTRL_SEC: NVMCTRL_SEC,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "OSCCTRL"]
    pub OSCCTRL: OSCCTRL,
    #[doc = "OSC32KCTRL"]
    pub OSC32KCTRL: OSC32KCTRL,
    #[doc = "PAC"]
    pub PAC: PAC,
    #[doc = "PAC_SEC"]
    pub PAC_SEC: PAC_SEC,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "PORT_SEC"]
    pub PORT_SEC: PORT_SEC,
    #[doc = "PORT_IOBUS"]
    pub PORT_IOBUS: PORT_IOBUS,
    #[doc = "PORT_IOBUS_SEC"]
    pub PORT_IOBUS_SEC: PORT_IOBUS_SEC,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TRAM"]
    pub TRAM: TRAM,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CCL: CCL {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EIC_SEC: EIC_SEC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            EVSYS_SEC: EVSYS_SEC {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            IDAU: IDAU {
                _marker: PhantomData,
            },
            MCLK: MCLK {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            NVMCTRL_SEC: NVMCTRL_SEC {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            OSCCTRL: OSCCTRL {
                _marker: PhantomData,
            },
            OSC32KCTRL: OSC32KCTRL {
                _marker: PhantomData,
            },
            PAC: PAC {
                _marker: PhantomData,
            },
            PAC_SEC: PAC_SEC {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            PORT_SEC: PORT_SEC {
                _marker: PhantomData,
            },
            PORT_IOBUS: PORT_IOBUS {
                _marker: PhantomData,
            },
            PORT_IOBUS_SEC: PORT_IOBUS_SEC {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TRAM: TRAM {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
        }
    }
}
