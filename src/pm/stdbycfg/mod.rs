#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::STDBYCFG {
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
#[doc = "Possible values of the field `PDCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDCFGR {
    #[doc = "PDSW power domain switching is handled by hardware."]
    DEFAULT,
    #[doc = "PDSW is forced ACTIVE."]
    PDSW,
}
impl PDCFGR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PDCFGR::DEFAULT => false,
            PDCFGR::PDSW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDCFGR {
        match value {
            false => PDCFGR::DEFAULT,
            true => PDCFGR::PDSW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == PDCFGR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `PDSW`"]
    #[inline]
    pub fn is_pdsw(&self) -> bool {
        *self == PDCFGR::PDSW
    }
}
#[doc = "Possible values of the field `DPGPDSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPGPDSWR {
    #[doc = "Dynamic Power Gating disabled"]
    _0,
    #[doc = "Dynamic Power Gating enabled"]
    _1,
}
impl DPGPDSWR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DPGPDSWR::_0 => false,
            DPGPDSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPGPDSWR {
        match value {
            false => DPGPDSWR::_0,
            true => DPGPDSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPGPDSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPGPDSWR::_1
    }
}
#[doc = "Possible values of the field `VREGSMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREGSMODR {
    #[doc = "Automatic mode"]
    AUTO,
    #[doc = "Performance oriented"]
    PERFORMANCE,
    #[doc = "Low Power oriented"]
    LP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VREGSMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VREGSMODR::AUTO => 0,
            VREGSMODR::PERFORMANCE => 1,
            VREGSMODR::LP => 2,
            VREGSMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VREGSMODR {
        match value {
            0 => VREGSMODR::AUTO,
            1 => VREGSMODR::PERFORMANCE,
            2 => VREGSMODR::LP,
            i => VREGSMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMODR::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMODR::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMODR::LP
    }
}
#[doc = r" Value of the field"]
pub struct BBIASHSR {
    bits: bool,
}
impl BBIASHSR {
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
#[doc = r" Value of the field"]
pub struct BBIASTRR {
    bits: bool,
}
impl BBIASTRR {
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
#[doc = "Values that can be written to the field `PDCFG`"]
pub enum PDCFGW {
    #[doc = "PDSW power domain switching is handled by hardware."]
    DEFAULT,
    #[doc = "PDSW is forced ACTIVE."]
    PDSW,
}
impl PDCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDCFGW::DEFAULT => false,
            PDCFGW::PDSW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PDCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDSW power domain switching is handled by hardware."]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(PDCFGW::DEFAULT)
    }
    #[doc = "PDSW is forced ACTIVE."]
    #[inline]
    pub fn pdsw(self) -> &'a mut W {
        self.variant(PDCFGW::PDSW)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPGPDSW`"]
pub enum DPGPDSWW {
    #[doc = "Dynamic Power Gating disabled"]
    _0,
    #[doc = "Dynamic Power Gating enabled"]
    _1,
}
impl DPGPDSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPGPDSWW::_0 => false,
            DPGPDSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPGPDSWW<'a> {
    w: &'a mut W,
}
impl<'a> _DPGPDSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPGPDSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dynamic Power Gating disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPGPDSWW::_0)
    }
    #[doc = "Dynamic Power Gating enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPGPDSWW::_1)
    }
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VREGSMOD`"]
pub enum VREGSMODW {
    #[doc = "Automatic mode"]
    AUTO,
    #[doc = "Performance oriented"]
    PERFORMANCE,
    #[doc = "Low Power oriented"]
    LP,
}
impl VREGSMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VREGSMODW::AUTO => 0,
            VREGSMODW::PERFORMANCE => 1,
            VREGSMODW::LP => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREGSMODW<'a> {
    w: &'a mut W,
}
impl<'a> _VREGSMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREGSMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Automatic mode"]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMODW::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMODW::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMODW::LP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BBIASHSW<'a> {
    w: &'a mut W,
}
impl<'a> _BBIASHSW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BBIASTRW<'a> {
    w: &'a mut W,
}
impl<'a> _BBIASTRW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline]
    pub fn pdcfg(&self) -> PDCFGR {
        PDCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline]
    pub fn dpgpdsw(&self) -> DPGPDSWR {
        DPGPDSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline]
    pub fn vregsmod(&self) -> VREGSMODR {
        VREGSMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline]
    pub fn bbiashs(&self) -> BBIASHSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BBIASHSR { bits }
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline]
    pub fn bbiastr(&self) -> BBIASTRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BBIASTRR { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline]
    pub fn pdcfg(&mut self) -> _PDCFGW {
        _PDCFGW { w: self }
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline]
    pub fn dpgpdsw(&mut self) -> _DPGPDSWW {
        _DPGPDSWW { w: self }
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline]
    pub fn vregsmod(&mut self) -> _VREGSMODW {
        _VREGSMODW { w: self }
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline]
    pub fn bbiashs(&mut self) -> _BBIASHSW {
        _BBIASHSW { w: self }
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline]
    pub fn bbiastr(&mut self) -> _BBIASTRW {
        _BBIASTRW { w: self }
    }
}
