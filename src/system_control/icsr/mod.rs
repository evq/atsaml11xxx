#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
pub struct VECTACTIVER {
    bits: u16,
}
impl VECTACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VECTPENDINGR {
    bits: u16,
}
impl VECTPENDINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISRPENDINGR {
    bits: bool,
}
impl ISRPENDINGR {
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
pub struct ISRPREEMPTR {
    bits: bool,
}
impl ISRPREEMPTR {
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
#[doc = "Possible values of the field `PENDSTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLRR {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the SysTick exception"]
    VALUE_1,
}
impl PENDSTCLRR {
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
            PENDSTCLRR::VALUE_0 => false,
            PENDSTCLRR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTCLRR {
        match value {
            false => PENDSTCLRR::VALUE_0,
            true => PENDSTCLRR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTCLRR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTCLRR::VALUE_1
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    VALUE_1,
}
impl PENDSTSETR {
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
            PENDSTSETR::VALUE_0 => false,
            PENDSTSETR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTSETR {
        match value {
            false => PENDSTSETR::VALUE_0,
            true => PENDSTSETR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTSETR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTSETR::VALUE_1
    }
}
#[doc = "Possible values of the field `PENDSVCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLRR {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the PendSV exception"]
    VALUE_1,
}
impl PENDSVCLRR {
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
            PENDSVCLRR::VALUE_0 => false,
            PENDSVCLRR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVCLRR {
        match value {
            false => PENDSVCLRR::VALUE_0,
            true => PENDSVCLRR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVCLRR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVCLRR::VALUE_1
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETR {
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    VALUE_1,
}
impl PENDSVSETR {
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
            PENDSVSETR::VALUE_0 => false,
            PENDSVSETR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVSETR {
        match value {
            false => PENDSVSETR::VALUE_0,
            true => PENDSVSETR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVSETR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVSETR::VALUE_1
    }
}
#[doc = "Possible values of the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSETR {
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    VALUE_1,
}
impl NMIPENDSETR {
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
            NMIPENDSETR::VALUE_0 => false,
            NMIPENDSETR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMIPENDSETR {
        match value {
            false => NMIPENDSETR::VALUE_0,
            true => NMIPENDSETR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == NMIPENDSETR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == NMIPENDSETR::VALUE_1
    }
}
#[doc = r" Proxy"]
pub struct _VECTACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTACTIVEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VECTPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTPENDINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISRPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ISRPENDINGW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISRPREEMPTW<'a> {
    w: &'a mut W,
}
impl<'a> _ISRPREEMPTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTCLR`"]
pub enum PENDSTCLRW {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the SysTick exception"]
    VALUE_1,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::VALUE_0 => false,
            PENDSTCLRW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE_0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::VALUE_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
pub enum PENDSTSETW {
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    VALUE_1,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::VALUE_0 => false,
            PENDSTSETW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE_0)
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTSETW::VALUE_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
pub enum PENDSVCLRW {
    #[doc = "No effect"]
    VALUE_0,
    #[doc = "Removes the pending state from the PendSV exception"]
    VALUE_1,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::VALUE_0 => false,
            PENDSVCLRW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE_0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::VALUE_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVSET`"]
pub enum PENDSVSETW {
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    VALUE_1,
}
impl PENDSVSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVSETW::VALUE_0 => false,
            PENDSVSETW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVSETW::VALUE_0)
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVSETW::VALUE_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NMIPENDSET`"]
pub enum NMIPENDSETW {
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    VALUE_0,
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    VALUE_1,
}
impl NMIPENDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMIPENDSETW::VALUE_0 => false,
            NMIPENDSETW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMIPENDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIPENDSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMIPENDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NMIPENDSETW::VALUE_0)
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NMIPENDSETW::VALUE_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:8 - Debug: Exception number of currently executing exception, or 0 if thread mode"]
    #[inline]
    pub fn vectactive(&self) -> VECTACTIVER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTACTIVER { bits }
    }
    #[doc = "Bits 12:20 - Exception number of the highest priority pending enabled exception"]
    #[inline]
    pub fn vectpending(&self) -> VECTPENDINGR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTPENDINGR { bits }
    }
    #[doc = "Bit 22 - Debug: NVIC interrupt pending"]
    #[inline]
    pub fn isrpending(&self) -> ISRPENDINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISRPENDINGR { bits }
    }
    #[doc = "Bit 23 - Debug: Pending exception serviced on exit from debug halt"]
    #[inline]
    pub fn isrpreempt(&self) -> ISRPREEMPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISRPREEMPTR { bits }
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline]
    pub fn pendstclr(&self) -> PENDSTCLRR {
        PENDSTCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&self) -> PENDSTSETR {
        PENDSTSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline]
    pub fn pendsvclr(&self) -> PENDSVCLRR {
        PENDSVCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline]
    pub fn pendsvset(&self) -> PENDSVSETR {
        PENDSVSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline]
    pub fn nmipendset(&self) -> NMIPENDSETR {
        NMIPENDSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:8 - Debug: Exception number of currently executing exception, or 0 if thread mode"]
    #[inline]
    pub fn vectactive(&mut self) -> _VECTACTIVEW {
        _VECTACTIVEW { w: self }
    }
    #[doc = "Bits 12:20 - Exception number of the highest priority pending enabled exception"]
    #[inline]
    pub fn vectpending(&mut self) -> _VECTPENDINGW {
        _VECTPENDINGW { w: self }
    }
    #[doc = "Bit 22 - Debug: NVIC interrupt pending"]
    #[inline]
    pub fn isrpending(&mut self) -> _ISRPENDINGW {
        _ISRPENDINGW { w: self }
    }
    #[doc = "Bit 23 - Debug: Pending exception serviced on exit from debug halt"]
    #[inline]
    pub fn isrpreempt(&mut self) -> _ISRPREEMPTW {
        _ISRPREEMPTW { w: self }
    }
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline]
    pub fn nmipendset(&mut self) -> _NMIPENDSETW {
        _NMIPENDSETW { w: self }
    }
}
