#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SLEEPCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SLEEPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPMODER {
    #[doc = "CPU, AHB, APB clocks are OFF"]
    IDLE,
    #[doc = "All Clocks are OFF"]
    STANDBY,
    #[doc = "All power domains are powered OFF"]
    OFF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLEEPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLEEPMODER::IDLE => 2,
            SLEEPMODER::STANDBY => 4,
            SLEEPMODER::OFF => 6,
            SLEEPMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLEEPMODER {
        match value {
            2 => SLEEPMODER::IDLE,
            4 => SLEEPMODER::STANDBY,
            6 => SLEEPMODER::OFF,
            i => SLEEPMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPMODER::IDLE
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODER::STANDBY
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODER::OFF
    }
}
#[doc = "Values that can be written to the field `SLEEPMODE`"]
pub enum SLEEPMODEW {
    #[doc = "CPU, AHB, APB clocks are OFF"]
    IDLE,
    #[doc = "All Clocks are OFF"]
    STANDBY,
    #[doc = "All power domains are powered OFF"]
    OFF,
}
impl SLEEPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLEEPMODEW::IDLE => 2,
            SLEEPMODEW::STANDBY => 4,
            SLEEPMODEW::OFF => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CPU, AHB, APB clocks are OFF"]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(SLEEPMODEW::IDLE)
    }
    #[doc = "All Clocks are OFF"]
    #[inline]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODEW::STANDBY)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODEW::OFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline]
    pub fn sleepmode(&self) -> SLEEPMODER {
        SLEEPMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline]
    pub fn sleepmode(&mut self) -> _SLEEPMODEW {
        _SLEEPMODEW { w: self }
    }
}
