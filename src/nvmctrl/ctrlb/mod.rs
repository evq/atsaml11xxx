#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLB {
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
pub struct RWSR {
    bits: u8,
}
impl RWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SLEEPPRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPPRMR {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS,
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT,
    #[doc = "Auto power reduction disabled."]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLEEPPRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLEEPPRMR::WAKEONACCESS => 0,
            SLEEPPRMR::WAKEUPINSTANT => 1,
            SLEEPPRMR::DISABLED => 3,
            SLEEPPRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLEEPPRMR {
        match value {
            0 => SLEEPPRMR::WAKEONACCESS,
            1 => SLEEPPRMR::WAKEUPINSTANT,
            3 => SLEEPPRMR::DISABLED,
            i => SLEEPPRMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
    #[inline]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == SLEEPPRMR::WAKEONACCESS
    }
    #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
    #[inline]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == SLEEPPRMR::WAKEUPINSTANT
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPPRMR::DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct FWUPR {
    bits: bool,
}
impl FWUPR {
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
#[doc = "Possible values of the field `READMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READMODER {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY,
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER,
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl READMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            READMODER::NO_MISS_PENALTY => 0,
            READMODER::LOW_POWER => 1,
            READMODER::DETERMINISTIC => 2,
            READMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> READMODER {
        match value {
            0 => READMODER::NO_MISS_PENALTY,
            1 => READMODER::LOW_POWER,
            2 => READMODER::DETERMINISTIC,
            i => READMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
    #[inline]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == READMODER::NO_MISS_PENALTY
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline]
    pub fn is_low_power(&self) -> bool {
        *self == READMODER::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
    #[inline]
    pub fn is_deterministic(&self) -> bool {
        *self == READMODER::DETERMINISTIC
    }
}
#[doc = r" Value of the field"]
pub struct CACHEDISR {
    bits: bool,
}
impl CACHEDISR {
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
pub struct QWENR {
    bits: bool,
}
impl QWENR {
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
#[doc = r" Proxy"]
pub struct _RWSW<'a> {
    w: &'a mut W,
}
impl<'a> _RWSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEPPRM`"]
pub enum SLEEPPRMW {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS,
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT,
    #[doc = "Auto power reduction disabled."]
    DISABLED,
}
impl SLEEPPRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLEEPPRMW::WAKEONACCESS => 0,
            SLEEPPRMW::WAKEUPINSTANT => 1,
            SLEEPPRMW::DISABLED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPPRMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPPRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPPRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline]
    pub fn wakeonaccess(self) -> &'a mut W {
        self.variant(SLEEPPRMW::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline]
    pub fn wakeupinstant(self) -> &'a mut W {
        self.variant(SLEEPPRMW::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPPRMW::DISABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FWUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FWUPW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `READMODE`"]
pub enum READMODEW {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY,
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER,
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC,
}
impl READMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            READMODEW::NO_MISS_PENALTY => 0,
            READMODEW::LOW_POWER => 1,
            READMODEW::DETERMINISTIC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _READMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline]
    pub fn no_miss_penalty(self) -> &'a mut W {
        self.variant(READMODEW::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline]
    pub fn low_power(self) -> &'a mut W {
        self.variant(READMODEW::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline]
    pub fn deterministic(self) -> &'a mut W {
        self.variant(READMODEW::DETERMINISTIC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CACHEDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEDISW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QWENW<'a> {
    w: &'a mut W,
}
impl<'a> _QWENW<'a> {
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&self) -> RWSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RWSR { bits }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn sleepprm(&self) -> SLEEPPRMR {
        SLEEPPRMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - fast wake-up"]
    #[inline]
    pub fn fwup(&self) -> FWUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FWUPR { bits }
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline]
    pub fn readmode(&self) -> READMODER {
        READMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline]
    pub fn cachedis(&self) -> CACHEDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CACHEDISR { bits }
    }
    #[doc = "Bit 19 - Quick Write Enable"]
    #[inline]
    pub fn qwen(&self) -> QWENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QWENR { bits }
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
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&mut self) -> _RWSW {
        _RWSW { w: self }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn sleepprm(&mut self) -> _SLEEPPRMW {
        _SLEEPPRMW { w: self }
    }
    #[doc = "Bit 11 - fast wake-up"]
    #[inline]
    pub fn fwup(&mut self) -> _FWUPW {
        _FWUPW { w: self }
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline]
    pub fn readmode(&mut self) -> _READMODEW {
        _READMODEW { w: self }
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline]
    pub fn cachedis(&mut self) -> _CACHEDISW {
        _CACHEDISW { w: self }
    }
    #[doc = "Bit 19 - Quick Write Enable"]
    #[inline]
    pub fn qwen(&mut self) -> _QWENW {
        _QWENW { w: self }
    }
}
