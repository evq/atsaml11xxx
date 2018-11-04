#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PLCFG {
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
#[doc = "Possible values of the field `PLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLSELR {
    #[doc = "Performance Level 0"]
    PL0,
    #[doc = "Performance Level 2"]
    PL2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLSELR::PL0 => 0,
            PLSELR::PL2 => 2,
            PLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLSELR {
        match value {
            0 => PLSELR::PL0,
            2 => PLSELR::PL2,
            i => PLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PL0`"]
    #[inline]
    pub fn is_pl0(&self) -> bool {
        *self == PLSELR::PL0
    }
    #[doc = "Checks if the value of the field is `PL2`"]
    #[inline]
    pub fn is_pl2(&self) -> bool {
        *self == PLSELR::PL2
    }
}
#[doc = r" Value of the field"]
pub struct PLDISR {
    bits: bool,
}
impl PLDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `PLSEL`"]
pub enum PLSELW {
    #[doc = "Performance Level 0"]
    PL0,
    #[doc = "Performance Level 2"]
    PL2,
}
impl PLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLSELW::PL0 => 0,
            PLSELW::PL2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Performance Level 0"]
    #[inline]
    pub fn pl0(self) -> &'a mut W {
        self.variant(PLSELW::PL0)
    }
    #[doc = "Performance Level 2"]
    #[inline]
    pub fn pl2(self) -> &'a mut W {
        self.variant(PLSELW::PL2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLDISW<'a> {
    w: &'a mut W,
}
impl<'a> _PLDISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline]
    pub fn plsel(&self) -> PLSELR {
        PLSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline]
    pub fn pldis(&self) -> PLDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        PLDISR { bits }
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
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline]
    pub fn plsel(&mut self) -> _PLSELW {
        _PLSELW { w: self }
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline]
    pub fn pldis(&mut self) -> _PLDISW {
        _PLDISW { w: self }
    }
}
