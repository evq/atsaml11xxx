#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `SLEEPONEXIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXITR {
    #[doc = "O not sleep when returning to Thread mode"]
    VALUE_0,
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    VALUE_1,
}
impl SLEEPONEXITR {
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
            SLEEPONEXITR::VALUE_0 => false,
            SLEEPONEXITR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPONEXITR {
        match value {
            false => SLEEPONEXITR::VALUE_0,
            true => SLEEPONEXITR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPONEXITR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPONEXITR::VALUE_1
    }
}
#[doc = "Possible values of the field `SLEEPDEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEPR {
    #[doc = "Sleep"]
    VALUE_0,
    #[doc = "Deep sleep"]
    VALUE_1,
}
impl SLEEPDEEPR {
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
            SLEEPDEEPR::VALUE_0 => false,
            SLEEPDEEPR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPDEEPR {
        match value {
            false => SLEEPDEEPR::VALUE_0,
            true => SLEEPDEEPR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SLEEPDEEPR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SLEEPDEEPR::VALUE_1
    }
}
#[doc = "Possible values of the field `SEVONPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPENDR {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    VALUE_0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    VALUE_1,
}
impl SEVONPENDR {
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
            SEVONPENDR::VALUE_0 => false,
            SEVONPENDR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEVONPENDR {
        match value {
            false => SEVONPENDR::VALUE_0,
            true => SEVONPENDR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SEVONPENDR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SEVONPENDR::VALUE_1
    }
}
#[doc = "Values that can be written to the field `SLEEPONEXIT`"]
pub enum SLEEPONEXITW {
    #[doc = "O not sleep when returning to Thread mode"]
    VALUE_0,
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    VALUE_1,
}
impl SLEEPONEXITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPONEXITW::VALUE_0 => false,
            SLEEPONEXITW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPONEXITW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPONEXITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPONEXITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "O not sleep when returning to Thread mode"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPONEXITW::VALUE_0)
    }
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SLEEPONEXITW::VALUE_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEPDEEP`"]
pub enum SLEEPDEEPW {
    #[doc = "Sleep"]
    VALUE_0,
    #[doc = "Deep sleep"]
    VALUE_1,
}
impl SLEEPDEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPDEEPW::VALUE_0 => false,
            SLEEPDEEPW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPDEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPDEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPDEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sleep"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SLEEPDEEPW::VALUE_0)
    }
    #[doc = "Deep sleep"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SLEEPDEEPW::VALUE_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEVONPEND`"]
pub enum SEVONPENDW {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    VALUE_0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    VALUE_1,
}
impl SEVONPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEVONPENDW::VALUE_0 => false,
            SEVONPENDW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEVONPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVONPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEVONPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SEVONPENDW::VALUE_0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SEVONPENDW::VALUE_1)
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
    #[doc = "Bit 1 - Sleep-On-Exit when exiting Handler mode"]
    #[inline]
    pub fn sleeponexit(&self) -> SLEEPONEXITR {
        SLEEPONEXITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Uses Deep Sleep as low power mode"]
    #[inline]
    pub fn sleepdeep(&self) -> SLEEPDEEPR {
        SLEEPDEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline]
    pub fn sevonpend(&self) -> SEVONPENDR {
        SEVONPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 1 - Sleep-On-Exit when exiting Handler mode"]
    #[inline]
    pub fn sleeponexit(&mut self) -> _SLEEPONEXITW {
        _SLEEPONEXITW { w: self }
    }
    #[doc = "Bit 2 - Uses Deep Sleep as low power mode"]
    #[inline]
    pub fn sleepdeep(&mut self) -> _SLEEPDEEPW {
        _SLEEPDEEPW { w: self }
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline]
    pub fn sevonpend(&mut self) -> _SEVONPENDW {
        _SEVONPENDW { w: self }
    }
}
