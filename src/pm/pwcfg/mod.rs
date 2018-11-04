#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PWCFG {
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
#[doc = "Possible values of the field `RAMPSWC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPSWCR {
    #[doc = "16KB Available"]
    _16KB,
    #[doc = "12KB Available"]
    _12KB,
    #[doc = "8KB Available"]
    _8KB,
    #[doc = "4KB Available"]
    _4KB,
}
impl RAMPSWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPSWCR::_16KB => 0,
            RAMPSWCR::_12KB => 1,
            RAMPSWCR::_8KB => 2,
            RAMPSWCR::_4KB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPSWCR {
        match value {
            0 => RAMPSWCR::_16KB,
            1 => RAMPSWCR::_12KB,
            2 => RAMPSWCR::_8KB,
            3 => RAMPSWCR::_4KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline]
    pub fn is_16kb(&self) -> bool {
        *self == RAMPSWCR::_16KB
    }
    #[doc = "Checks if the value of the field is `_12KB`"]
    #[inline]
    pub fn is_12kb(&self) -> bool {
        *self == RAMPSWCR::_12KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == RAMPSWCR::_8KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == RAMPSWCR::_4KB
    }
}
#[doc = "Values that can be written to the field `RAMPSWC`"]
pub enum RAMPSWCW {
    #[doc = "16KB Available"]
    _16KB,
    #[doc = "12KB Available"]
    _12KB,
    #[doc = "8KB Available"]
    _8KB,
    #[doc = "4KB Available"]
    _4KB,
}
impl RAMPSWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPSWCW::_16KB => 0,
            RAMPSWCW::_12KB => 1,
            RAMPSWCW::_8KB => 2,
            RAMPSWCW::_4KB => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPSWCW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPSWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPSWCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "16KB Available"]
    #[inline]
    pub fn _16kb(self) -> &'a mut W {
        self.variant(RAMPSWCW::_16KB)
    }
    #[doc = "12KB Available"]
    #[inline]
    pub fn _12kb(self) -> &'a mut W {
        self.variant(RAMPSWCW::_12KB)
    }
    #[doc = "8KB Available"]
    #[inline]
    pub fn _8kb(self) -> &'a mut W {
        self.variant(RAMPSWCW::_8KB)
    }
    #[doc = "4KB Available"]
    #[inline]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(RAMPSWCW::_4KB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline]
    pub fn rampswc(&self) -> RAMPSWCR {
        RAMPSWCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline]
    pub fn rampswc(&mut self) -> _RAMPSWCW {
        _RAMPSWCW { w: self }
    }
}
