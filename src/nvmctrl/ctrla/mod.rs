#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLA {
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
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    ER,
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP,
    #[doc = "Sets the power reduction mode."]
    SPRM,
    #[doc = "Clears the power reduction mode."]
    CPRM,
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    PBC,
    #[doc = "Invalidate all cache lines."]
    INVALL,
    #[doc = "Set DAL=0"]
    SDAL0,
    #[doc = "Set DAL=1"]
    SDAL1,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::ER => 2,
            CMDW::WP => 4,
            CMDW::SPRM => 66,
            CMDW::CPRM => 67,
            CMDW::PBC => 68,
            CMDW::INVALL => 70,
            CMDW::SDAL0 => 75,
            CMDW::SDAL1 => 76,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline]
    pub fn er(self) -> &'a mut W {
        self.variant(CMDW::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDW::WP)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMDW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMDW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMDW::PBC)
    }
    #[doc = "Invalidate all cache lines."]
    #[inline]
    pub fn invall(self) -> &'a mut W {
        self.variant(CMDW::INVALL)
    }
    #[doc = "Set DAL=0"]
    #[inline]
    pub fn sdal0(self) -> &'a mut W {
        self.variant(CMDW::SDAL0)
    }
    #[doc = "Set DAL=1"]
    #[inline]
    pub fn sdal1(self) -> &'a mut W {
        self.variant(CMDW::SDAL1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDEX`"]
pub enum CMDEXW {
    #[doc = "Execution Key"]
    KEY,
}
impl CMDEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDEXW::KEY => 165,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDEXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Execution Key"]
    #[inline]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEXW::KEY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
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
    #[doc = "Bits 0:6 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline]
    pub fn cmdex(&mut self) -> _CMDEXW {
        _CMDEXW { w: self }
    }
}
