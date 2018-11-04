#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::STATUSB {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DALR {
    #[doc = "undocumented"]
    SECURED,
    #[doc = "undocumented"]
    NS_DEBUG,
    #[doc = "undocumented"]
    FULL_DEBUG,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DALR::SECURED => 0,
            DALR::NS_DEBUG => 1,
            DALR::FULL_DEBUG => 2,
            DALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DALR {
        match value {
            0 => DALR::SECURED,
            1 => DALR::NS_DEBUG,
            2 => DALR::FULL_DEBUG,
            i => DALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SECURED`"]
    #[inline]
    pub fn is_secured(&self) -> bool {
        *self == DALR::SECURED
    }
    #[doc = "Checks if the value of the field is `NS_DEBUG`"]
    #[inline]
    pub fn is_ns_debug(&self) -> bool {
        *self == DALR::NS_DEBUG
    }
    #[doc = "Checks if the value of the field is `FULL_DEBUG`"]
    #[inline]
    pub fn is_full_debug(&self) -> bool {
        *self == DALR::FULL_DEBUG
    }
}
#[doc = r" Value of the field"]
pub struct DBGPRESR {
    bits: bool,
}
impl DBGPRESR {
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
pub struct HPER {
    bits: bool,
}
impl HPER {
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
pub struct DCCD0R {
    bits: bool,
}
impl DCCD0R {
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
pub struct DCCD1R {
    bits: bool,
}
impl DCCD1R {
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
pub struct BCCD0R {
    bits: bool,
}
impl BCCD0R {
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
pub struct BCCD1R {
    bits: bool,
}
impl BCCD1R {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Debugger Access Level"]
    #[inline]
    pub fn dal(&self) -> DALR {
        DALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - Debugger Present"]
    #[inline]
    pub fn dbgpres(&self) -> DBGPRESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DBGPRESR { bits }
    }
    #[doc = "Bit 3 - Hot-Plugging Enable"]
    #[inline]
    pub fn hpe(&self) -> HPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        HPER { bits }
    }
    #[doc = "Bit 4 - Debug Communication Channel 0 Dirty"]
    #[inline]
    pub fn dccd0(&self) -> DCCD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DCCD0R { bits }
    }
    #[doc = "Bit 5 - Debug Communication Channel 1 Dirty"]
    #[inline]
    pub fn dccd1(&self) -> DCCD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DCCD1R { bits }
    }
    #[doc = "Bit 6 - Boot ROM Communication Channel 0 Dirty"]
    #[inline]
    pub fn bccd0(&self) -> BCCD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        BCCD0R { bits }
    }
    #[doc = "Bit 7 - Boot ROM Communication Channel 1 Dirty"]
    #[inline]
    pub fn bccd1(&self) -> BCCD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        BCCD1R { bits }
    }
}
