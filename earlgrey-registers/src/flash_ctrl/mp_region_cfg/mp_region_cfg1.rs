#[doc = "Reader of register MP_REGION_CFG1"]
pub type R = crate::R<u32, super::MP_REGION_CFG1>;
#[doc = "Writer for register MP_REGION_CFG1"]
pub type W = crate::W<u32, super::MP_REGION_CFG1>;
#[doc = "Register MP_REGION_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN1`"]
pub type EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN1`"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
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
#[doc = "Reader of field `RD_EN1`"]
pub type RD_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN1`"]
pub struct RD_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN1_W<'a> {
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
#[doc = "Reader of field `PROG_EN1`"]
pub type PROG_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN1`"]
pub struct PROG_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BASE1`"]
pub type BASE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE1`"]
pub struct BASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE1`"]
pub type SIZE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE1`"]
pub struct SIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL1"]
    #[inline(always)]
    pub fn rd_en1(&self) -> RD_EN1_R {
        RD_EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL1"]
    #[inline(always)]
    pub fn prog_en1(&self) -> PROG_EN1_R {
        PROG_EN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL1"]
    #[inline(always)]
    pub fn erase_en1(&self) -> ERASE_EN1_R {
        ERASE_EN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL1"]
    #[inline(always)]
    pub fn base1(&self) -> BASE1_R {
        BASE1_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL1"]
    #[inline(always)]
    pub fn size1(&self) -> SIZE1_R {
        SIZE1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL1"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL1"]
    #[inline(always)]
    pub fn rd_en1(&mut self) -> RD_EN1_W {
        RD_EN1_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL1"]
    #[inline(always)]
    pub fn prog_en1(&mut self) -> PROG_EN1_W {
        PROG_EN1_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL1"]
    #[inline(always)]
    pub fn erase_en1(&mut self) -> ERASE_EN1_W {
        ERASE_EN1_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL1"]
    #[inline(always)]
    pub fn base1(&mut self) -> BASE1_W {
        BASE1_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL1"]
    #[inline(always)]
    pub fn size1(&mut self) -> SIZE1_W {
        SIZE1_W { w: self }
    }
}
