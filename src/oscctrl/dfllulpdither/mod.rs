#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DFLLULPDITHER {
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
#[doc = "Possible values of the field `STEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEPR {
    #[doc = "Dither Step = 1"]
    STEP1,
    #[doc = "Dither Step = 2"]
    STEP2,
    #[doc = "Dither Step = 4"]
    STEP4,
    #[doc = "Dither Step = 8"]
    STEP8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STEPR::STEP1 => 0,
            STEPR::STEP2 => 1,
            STEPR::STEP4 => 2,
            STEPR::STEP8 => 3,
            STEPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STEPR {
        match value {
            0 => STEPR::STEP1,
            1 => STEPR::STEP2,
            2 => STEPR::STEP4,
            3 => STEPR::STEP8,
            i => STEPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STEP1`"]
    #[inline]
    pub fn is_step1(&self) -> bool {
        *self == STEPR::STEP1
    }
    #[doc = "Checks if the value of the field is `STEP2`"]
    #[inline]
    pub fn is_step2(&self) -> bool {
        *self == STEPR::STEP2
    }
    #[doc = "Checks if the value of the field is `STEP4`"]
    #[inline]
    pub fn is_step4(&self) -> bool {
        *self == STEPR::STEP4
    }
    #[doc = "Checks if the value of the field is `STEP8`"]
    #[inline]
    pub fn is_step8(&self) -> bool {
        *self == STEPR::STEP8
    }
}
#[doc = "Possible values of the field `PER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR {
    #[doc = "Dither Over 1 Reference Clock Period"]
    PER1,
    #[doc = "Dither Over 2 Reference Clock Period"]
    PER2,
    #[doc = "Dither Over 4 Reference Clock Period"]
    PER4,
    #[doc = "Dither Over 8 Reference Clock Period"]
    PER8,
    #[doc = "Dither Over 16 Reference Clock Period"]
    PER16,
    #[doc = "Dither Over 32 Reference Clock Period"]
    PER32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERR::PER1 => 0,
            PERR::PER2 => 1,
            PERR::PER4 => 2,
            PERR::PER8 => 3,
            PERR::PER16 => 4,
            PERR::PER32 => 5,
            PERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERR {
        match value {
            0 => PERR::PER1,
            1 => PERR::PER2,
            2 => PERR::PER4,
            3 => PERR::PER8,
            4 => PERR::PER16,
            5 => PERR::PER32,
            i => PERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PER1`"]
    #[inline]
    pub fn is_per1(&self) -> bool {
        *self == PERR::PER1
    }
    #[doc = "Checks if the value of the field is `PER2`"]
    #[inline]
    pub fn is_per2(&self) -> bool {
        *self == PERR::PER2
    }
    #[doc = "Checks if the value of the field is `PER4`"]
    #[inline]
    pub fn is_per4(&self) -> bool {
        *self == PERR::PER4
    }
    #[doc = "Checks if the value of the field is `PER8`"]
    #[inline]
    pub fn is_per8(&self) -> bool {
        *self == PERR::PER8
    }
    #[doc = "Checks if the value of the field is `PER16`"]
    #[inline]
    pub fn is_per16(&self) -> bool {
        *self == PERR::PER16
    }
    #[doc = "Checks if the value of the field is `PER32`"]
    #[inline]
    pub fn is_per32(&self) -> bool {
        *self == PERR::PER32
    }
}
#[doc = "Values that can be written to the field `STEP`"]
pub enum STEPW {
    #[doc = "Dither Step = 1"]
    STEP1,
    #[doc = "Dither Step = 2"]
    STEP2,
    #[doc = "Dither Step = 4"]
    STEP4,
    #[doc = "Dither Step = 8"]
    STEP8,
}
impl STEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STEPW::STEP1 => 0,
            STEPW::STEP2 => 1,
            STEPW::STEP4 => 2,
            STEPW::STEP8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _STEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STEPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dither Step = 1"]
    #[inline]
    pub fn step1(self) -> &'a mut W {
        self.variant(STEPW::STEP1)
    }
    #[doc = "Dither Step = 2"]
    #[inline]
    pub fn step2(self) -> &'a mut W {
        self.variant(STEPW::STEP2)
    }
    #[doc = "Dither Step = 4"]
    #[inline]
    pub fn step4(self) -> &'a mut W {
        self.variant(STEPW::STEP4)
    }
    #[doc = "Dither Step = 8"]
    #[inline]
    pub fn step8(self) -> &'a mut W {
        self.variant(STEPW::STEP8)
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
#[doc = "Values that can be written to the field `PER`"]
pub enum PERW {
    #[doc = "Dither Over 1 Reference Clock Period"]
    PER1,
    #[doc = "Dither Over 2 Reference Clock Period"]
    PER2,
    #[doc = "Dither Over 4 Reference Clock Period"]
    PER4,
    #[doc = "Dither Over 8 Reference Clock Period"]
    PER8,
    #[doc = "Dither Over 16 Reference Clock Period"]
    PER16,
    #[doc = "Dither Over 32 Reference Clock Period"]
    PER32,
}
impl PERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERW::PER1 => 0,
            PERW::PER2 => 1,
            PERW::PER4 => 2,
            PERW::PER8 => 3,
            PERW::PER16 => 4,
            PERW::PER32 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERW<'a> {
    w: &'a mut W,
}
impl<'a> _PERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dither Over 1 Reference Clock Period"]
    #[inline]
    pub fn per1(self) -> &'a mut W {
        self.variant(PERW::PER1)
    }
    #[doc = "Dither Over 2 Reference Clock Period"]
    #[inline]
    pub fn per2(self) -> &'a mut W {
        self.variant(PERW::PER2)
    }
    #[doc = "Dither Over 4 Reference Clock Period"]
    #[inline]
    pub fn per4(self) -> &'a mut W {
        self.variant(PERW::PER4)
    }
    #[doc = "Dither Over 8 Reference Clock Period"]
    #[inline]
    pub fn per8(self) -> &'a mut W {
        self.variant(PERW::PER8)
    }
    #[doc = "Dither Over 16 Reference Clock Period"]
    #[inline]
    pub fn per16(self) -> &'a mut W {
        self.variant(PERW::PER16)
    }
    #[doc = "Dither Over 32 Reference Clock Period"]
    #[inline]
    pub fn per32(self) -> &'a mut W {
        self.variant(PERW::PER32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline]
    pub fn step(&self) -> STEPR {
        STEPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline]
    pub fn per(&self) -> PERR {
        PERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline]
    pub fn step(&mut self) -> _STEPW {
        _STEPW { w: self }
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline]
    pub fn per(&mut self) -> _PERW {
        _PERW { w: self }
    }
}
