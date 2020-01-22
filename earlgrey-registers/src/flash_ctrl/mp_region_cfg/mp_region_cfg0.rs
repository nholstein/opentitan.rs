#[doc = "Reader of register MP_REGION_CFG0"]
pub type R = crate::R<u32, super::MP_REGION_CFG0>;
#[doc = "Writer for register MP_REGION_CFG0"]
pub type W = crate::W<u32, super::MP_REGION_CFG0>;
#[doc = "Register MP_REGION_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN0`"]
pub type EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN0`"]
pub struct EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN0_W<'a> {
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
#[doc = "Reader of field `RD_EN0`"]
pub type RD_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN0`"]
pub struct RD_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN0_W<'a> {
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
#[doc = "Reader of field `PROG_EN0`"]
pub type PROG_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN0`"]
pub struct PROG_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BASE0`"]
pub type BASE0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE0`"]
pub struct BASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE0`"]
pub type SIZE0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE0`"]
pub struct SIZE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL0"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL0"]
    #[inline(always)]
    pub fn rd_en0(&self) -> RD_EN0_R {
        RD_EN0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL0"]
    #[inline(always)]
    pub fn prog_en0(&self) -> PROG_EN0_R {
        PROG_EN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL0"]
    #[inline(always)]
    pub fn erase_en0(&self) -> ERASE_EN0_R {
        ERASE_EN0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL0"]
    #[inline(always)]
    pub fn base0(&self) -> BASE0_R {
        BASE0_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL0"]
    #[inline(always)]
    pub fn size0(&self) -> SIZE0_R {
        SIZE0_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL0"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W {
        EN0_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL0"]
    #[inline(always)]
    pub fn rd_en0(&mut self) -> RD_EN0_W {
        RD_EN0_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL0"]
    #[inline(always)]
    pub fn prog_en0(&mut self) -> PROG_EN0_W {
        PROG_EN0_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL0"]
    #[inline(always)]
    pub fn erase_en0(&mut self) -> ERASE_EN0_W {
        ERASE_EN0_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL0"]
    #[inline(always)]
    pub fn base0(&mut self) -> BASE0_W {
        BASE0_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL0"]
    #[inline(always)]
    pub fn size0(&mut self) -> SIZE0_W {
        SIZE0_W { w: self }
    }
}
