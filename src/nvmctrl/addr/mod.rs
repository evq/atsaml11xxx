#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDR {
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
pub struct AOFFSETR {
    bits: u16,
}
impl AOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRAYR {
    #[doc = "FLASH Array"]
    FLASH,
    #[doc = "DATA FLASH Array"]
    DATAFLASH,
    #[doc = "Auxilliary Space"]
    AUX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARRAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARRAYR::FLASH => 0,
            ARRAYR::DATAFLASH => 1,
            ARRAYR::AUX => 2,
            ARRAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARRAYR {
        match value {
            0 => ARRAYR::FLASH,
            1 => ARRAYR::DATAFLASH,
            2 => ARRAYR::AUX,
            i => ARRAYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == ARRAYR::FLASH
    }
    #[doc = "Checks if the value of the field is `DATAFLASH`"]
    #[inline]
    pub fn is_dataflash(&self) -> bool {
        *self == ARRAYR::DATAFLASH
    }
    #[doc = "Checks if the value of the field is `AUX`"]
    #[inline]
    pub fn is_aux(&self) -> bool {
        *self == ARRAYR::AUX
    }
}
#[doc = r" Proxy"]
pub struct _AOFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _AOFFSETW<'a> {
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
#[doc = "Values that can be written to the field `ARRAY`"]
pub enum ARRAYW {
    #[doc = "FLASH Array"]
    FLASH,
    #[doc = "DATA FLASH Array"]
    DATAFLASH,
    #[doc = "Auxilliary Space"]
    AUX,
}
impl ARRAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARRAYW::FLASH => 0,
            ARRAYW::DATAFLASH => 1,
            ARRAYW::AUX => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARRAYW<'a> {
    w: &'a mut W,
}
impl<'a> _ARRAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARRAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FLASH Array"]
    #[inline]
    pub fn flash(self) -> &'a mut W {
        self.variant(ARRAYW::FLASH)
    }
    #[doc = "DATA FLASH Array"]
    #[inline]
    pub fn dataflash(self) -> &'a mut W {
        self.variant(ARRAYW::DATAFLASH)
    }
    #[doc = "Auxilliary Space"]
    #[inline]
    pub fn aux(self) -> &'a mut W {
        self.variant(ARRAYW::AUX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline]
    pub fn aoffset(&self) -> AOFFSETR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AOFFSETR { bits }
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline]
    pub fn array(&self) -> ARRAYR {
        ARRAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline]
    pub fn aoffset(&mut self) -> _AOFFSETW {
        _AOFFSETW { w: self }
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline]
    pub fn array(&mut self) -> _ARRAYW {
        _ARRAYW { w: self }
    }
}
