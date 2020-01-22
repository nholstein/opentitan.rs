#[doc = "Reader of register MP_REGION_CFG6"]
pub type R = crate::R<u32, super::MP_REGION_CFG6>;
#[doc = "Writer for register MP_REGION_CFG6"]
pub type W = crate::W<u32, super::MP_REGION_CFG6>;
#[doc = "Register MP_REGION_CFG6 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN6`"]
pub type EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN6`"]
pub struct EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EN6_W<'a> {
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
#[doc = "Reader of field `RD_EN6`"]
pub type RD_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN6`"]
pub struct RD_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN6_W<'a> {
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
#[doc = "Reader of field `PROG_EN6`"]
pub type PROG_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN6`"]
pub struct PROG_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN6_W<'a> {
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
#[doc = "Reader of field `ERASE_EN6`"]
pub type ERASE_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN6`"]
pub struct ERASE_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN6_W<'a> {
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
#[doc = "Reader of field `BASE6`"]
pub type BASE6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE6`"]
pub struct BASE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE6`"]
pub type SIZE6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE6`"]
pub struct SIZE6_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL6"]
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL6"]
    #[inline(always)]
    pub fn rd_en6(&self) -> RD_EN6_R {
        RD_EN6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL6"]
    #[inline(always)]
    pub fn prog_en6(&self) -> PROG_EN6_R {
        PROG_EN6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL6"]
    #[inline(always)]
    pub fn erase_en6(&self) -> ERASE_EN6_R {
        ERASE_EN6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL6"]
    #[inline(always)]
    pub fn base6(&self) -> BASE6_R {
        BASE6_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL6"]
    #[inline(always)]
    pub fn size6(&self) -> SIZE6_R {
        SIZE6_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL6"]
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W {
        EN6_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL6"]
    #[inline(always)]
    pub fn rd_en6(&mut self) -> RD_EN6_W {
        RD_EN6_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL6"]
    #[inline(always)]
    pub fn prog_en6(&mut self) -> PROG_EN6_W {
        PROG_EN6_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL6"]
    #[inline(always)]
    pub fn erase_en6(&mut self) -> ERASE_EN6_W {
        ERASE_EN6_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL6"]
    #[inline(always)]
    pub fn base6(&mut self) -> BASE6_W {
        BASE6_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL6"]
    #[inline(always)]
    pub fn size6(&mut self) -> SIZE6_W {
        SIZE6_W { w: self }
    }
}
