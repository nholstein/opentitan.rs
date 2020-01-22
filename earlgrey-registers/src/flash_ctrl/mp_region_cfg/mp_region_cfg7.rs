#[doc = "Reader of register MP_REGION_CFG7"]
pub type R = crate::R<u32, super::MP_REGION_CFG7>;
#[doc = "Writer for register MP_REGION_CFG7"]
pub type W = crate::W<u32, super::MP_REGION_CFG7>;
#[doc = "Register MP_REGION_CFG7 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN7`"]
pub type EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN7`"]
pub struct EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EN7_W<'a> {
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
#[doc = "Reader of field `RD_EN7`"]
pub type RD_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN7`"]
pub struct RD_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN7_W<'a> {
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
#[doc = "Reader of field `PROG_EN7`"]
pub type PROG_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN7`"]
pub struct PROG_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN7_W<'a> {
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
#[doc = "Reader of field `ERASE_EN7`"]
pub type ERASE_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN7`"]
pub struct ERASE_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN7_W<'a> {
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
#[doc = "Reader of field `BASE7`"]
pub type BASE7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE7`"]
pub struct BASE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE7`"]
pub type SIZE7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE7`"]
pub struct SIZE7_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL7"]
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL7"]
    #[inline(always)]
    pub fn rd_en7(&self) -> RD_EN7_R {
        RD_EN7_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL7"]
    #[inline(always)]
    pub fn prog_en7(&self) -> PROG_EN7_R {
        PROG_EN7_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL7"]
    #[inline(always)]
    pub fn erase_en7(&self) -> ERASE_EN7_R {
        ERASE_EN7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL7"]
    #[inline(always)]
    pub fn base7(&self) -> BASE7_R {
        BASE7_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL7"]
    #[inline(always)]
    pub fn size7(&self) -> SIZE7_R {
        SIZE7_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL7"]
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W {
        EN7_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL7"]
    #[inline(always)]
    pub fn rd_en7(&mut self) -> RD_EN7_W {
        RD_EN7_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL7"]
    #[inline(always)]
    pub fn prog_en7(&mut self) -> PROG_EN7_W {
        PROG_EN7_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL7"]
    #[inline(always)]
    pub fn erase_en7(&mut self) -> ERASE_EN7_W {
        ERASE_EN7_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL7"]
    #[inline(always)]
    pub fn base7(&mut self) -> BASE7_W {
        BASE7_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL7"]
    #[inline(always)]
    pub fn size7(&mut self) -> SIZE7_W {
        SIZE7_W { w: self }
    }
}
