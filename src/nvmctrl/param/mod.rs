#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PARAM {
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
#[doc = r" Value of the field"]
pub struct FLASHPR {
    bits: u16,
}
impl FLASHPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSZR {
    #[doc = "8 bytes"]
    _8,
    #[doc = "16 bytes"]
    _16,
    #[doc = "32 bytes"]
    _32,
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "256 bytes"]
    _256,
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
}
impl PSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSZR::_8 => 0,
            PSZR::_16 => 1,
            PSZR::_32 => 2,
            PSZR::_64 => 3,
            PSZR::_128 => 4,
            PSZR::_256 => 5,
            PSZR::_512 => 6,
            PSZR::_1024 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSZR {
        match value {
            0 => PSZR::_8,
            1 => PSZR::_16,
            2 => PSZR::_32,
            3 => PSZR::_64,
            4 => PSZR::_128,
            5 => PSZR::_256,
            6 => PSZR::_512,
            7 => PSZR::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PSZR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PSZR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PSZR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PSZR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PSZR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == PSZR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == PSZR::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == PSZR::_1024
    }
}
#[doc = r" Value of the field"]
pub struct DFLASHPR {
    bits: u16,
}
impl DFLASHPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FLASHPW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSZ`"]
pub enum PSZW {
    #[doc = "8 bytes"]
    _8,
    #[doc = "16 bytes"]
    _16,
    #[doc = "32 bytes"]
    _32,
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "256 bytes"]
    _256,
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
}
impl PSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSZW::_8 => 0,
            PSZW::_16 => 1,
            PSZW::_32 => 2,
            PSZW::_64 => 3,
            PSZW::_128 => 4,
            PSZW::_256 => 5,
            PSZW::_512 => 6,
            PSZW::_1024 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSZW<'a> {
    w: &'a mut W,
}
impl<'a> _PSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bytes"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSZW::_8)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSZW::_16)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSZW::_32)
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSZW::_64)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSZW::_128)
    }
    #[doc = "256 bytes"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSZW::_256)
    }
    #[doc = "512 bytes"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSZW::_512)
    }
    #[doc = "1024 bytes"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSZW::_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLASHPW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLASHPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline]
    pub fn flashp(&self) -> FLASHPR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FLASHPR { bits }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline]
    pub fn psz(&self) -> PSZR {
        PSZR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline]
    pub fn dflashp(&self) -> DFLASHPR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DFLASHPR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline]
    pub fn flashp(&mut self) -> _FLASHPW {
        _FLASHPW { w: self }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline]
    pub fn psz(&mut self) -> _PSZW {
        _PSZW { w: self }
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline]
    pub fn dflashp(&mut self) -> _DFLASHPW {
        _DFLASHPW { w: self }
    }
}
