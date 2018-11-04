#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPUID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct REVISIONR {
    bits: u8,
}
impl REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PARTNOR {
    bits: u16,
}
impl PARTNOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ARCHITECTURER {
    bits: u8,
}
impl ARCHITECTURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VARIANTR {
    bits: u8,
}
impl VARIANTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMPLEMENTERR {
    bits: u8,
}
impl IMPLEMENTERR {
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
    #[doc = "Bits 0:3 - Minor revision number"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVISIONR { bits }
    }
    #[doc = "Bits 4:15 - Processor Part Number, 0xD20=Cortex-M23"]
    #[inline]
    pub fn partno(&self) -> PARTNOR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PARTNOR { bits }
    }
    #[doc = "Bits 16:19 - Processor Architecture, 0xC=ARMv8-M BL"]
    #[inline]
    pub fn architecture(&self) -> ARCHITECTURER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARCHITECTURER { bits }
    }
    #[doc = "Bits 20:23 - Major revision number"]
    #[inline]
    pub fn variant(&self) -> VARIANTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VARIANTR { bits }
    }
    #[doc = "Bits 24:31 - Implementer code, ARM=0x41"]
    #[inline]
    pub fn implementer(&self) -> IMPLEMENTERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPLEMENTERR { bits }
    }
}
