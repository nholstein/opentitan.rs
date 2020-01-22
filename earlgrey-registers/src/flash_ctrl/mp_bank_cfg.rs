#[doc = "Reader of register MP_BANK_CFG"]
pub type R = crate::R<u32, super::MP_BANK_CFG>;
#[doc = "Writer for register MP_BANK_CFG"]
pub type W = crate::W<u32, super::MP_BANK_CFG>;
#[doc = "Register MP_BANK_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_BANK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERASE_EN0`"]
pub type ERASE_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN0`"]
pub struct ERASE_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ERASE_EN1`"]
pub type ERASE_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN1`"]
pub struct ERASE_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bank wide erase enable for FLASH_CTRL0"]
    #[inline(always)]
    pub fn erase_en0(&self) -> ERASE_EN0_R {
        ERASE_EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for FLASH_CTRL1"]
    #[inline(always)]
    pub fn erase_en1(&self) -> ERASE_EN1_R {
        ERASE_EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank wide erase enable for FLASH_CTRL0"]
    #[inline(always)]
    pub fn erase_en0(&mut self) -> ERASE_EN0_W {
        ERASE_EN0_W { w: self }
    }
    #[doc = "Bit 1 - for FLASH_CTRL1"]
    #[inline(always)]
    pub fn erase_en1(&mut self) -> ERASE_EN1_W {
        ERASE_EN1_W { w: self }
    }
}
