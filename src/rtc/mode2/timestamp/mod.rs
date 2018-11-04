#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TIMESTAMP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SECONDR {
    bits: u8,
}
impl SECONDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINUTER {
    bits: u8,
}
impl MINUTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HOURR {
    bits: u8,
}
impl HOURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DAYR {
    bits: u8,
}
impl DAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MONTHR {
    bits: u8,
}
impl MONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct YEARR {
    bits: u8,
}
impl YEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Second Timestamp Value"]
    #[inline]
    pub fn second(&self) -> SECONDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECONDR { bits }
    }
    #[doc = "Bits 6:11 - Minute Timestamp Value"]
    #[inline]
    pub fn minute(&self) -> MINUTER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINUTER { bits }
    }
    #[doc = "Bits 12:16 - Hour Timestamp Value"]
    #[inline]
    pub fn hour(&self) -> HOURR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HOURR { bits }
    }
    #[doc = "Bits 17:21 - Day Timestamp Value"]
    #[inline]
    pub fn day(&self) -> DAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAYR { bits }
    }
    #[doc = "Bits 22:25 - Month Timestamp Value"]
    #[inline]
    pub fn month(&self) -> MONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MONTHR { bits }
    }
    #[doc = "Bits 26:31 - Year Timestamp Value"]
    #[inline]
    pub fn year(&self) -> YEARR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        YEARR { bits }
    }
}
