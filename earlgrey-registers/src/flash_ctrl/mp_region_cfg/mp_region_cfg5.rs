#[doc = "Reader of register MP_REGION_CFG5"]
pub type R = crate::R<u32, super::MP_REGION_CFG5>;
#[doc = "Writer for register MP_REGION_CFG5"]
pub type W = crate::W<u32, super::MP_REGION_CFG5>;
#[doc = "Register MP_REGION_CFG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN5`"]
pub type EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN5`"]
pub struct EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EN5_W<'a> {
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
#[doc = "Reader of field `RD_EN5`"]
pub type RD_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN5`"]
pub struct RD_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN5_W<'a> {
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
#[doc = "Reader of field `PROG_EN5`"]
pub type PROG_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN5`"]
pub struct PROG_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN5_W<'a> {
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
#[doc = "Reader of field `ERASE_EN5`"]
pub type ERASE_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN5`"]
pub struct ERASE_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN5_W<'a> {
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
#[doc = "Reader of field `BASE5`"]
pub type BASE5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE5`"]
pub struct BASE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE5`"]
pub type SIZE5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE5`"]
pub struct SIZE5_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL5"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL5"]
    #[inline(always)]
    pub fn rd_en5(&self) -> RD_EN5_R {
        RD_EN5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL5"]
    #[inline(always)]
    pub fn prog_en5(&self) -> PROG_EN5_R {
        PROG_EN5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL5"]
    #[inline(always)]
    pub fn erase_en5(&self) -> ERASE_EN5_R {
        ERASE_EN5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL5"]
    #[inline(always)]
    pub fn base5(&self) -> BASE5_R {
        BASE5_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL5"]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL5"]
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W {
        EN5_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL5"]
    #[inline(always)]
    pub fn rd_en5(&mut self) -> RD_EN5_W {
        RD_EN5_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL5"]
    #[inline(always)]
    pub fn prog_en5(&mut self) -> PROG_EN5_W {
        PROG_EN5_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL5"]
    #[inline(always)]
    pub fn erase_en5(&mut self) -> ERASE_EN5_W {
        ERASE_EN5_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL5"]
    #[inline(always)]
    pub fn base5(&mut self) -> BASE5_W {
        BASE5_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL5"]
    #[inline(always)]
    pub fn size5(&mut self) -> SIZE5_W {
        SIZE5_W { w: self }
    }
}
