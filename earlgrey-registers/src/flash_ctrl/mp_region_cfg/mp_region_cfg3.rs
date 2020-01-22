#[doc = "Reader of register MP_REGION_CFG3"]
pub type R = crate::R<u32, super::MP_REGION_CFG3>;
#[doc = "Writer for register MP_REGION_CFG3"]
pub type W = crate::W<u32, super::MP_REGION_CFG3>;
#[doc = "Register MP_REGION_CFG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN3`"]
pub type EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN3`"]
pub struct EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN3_W<'a> {
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
#[doc = "Reader of field `RD_EN3`"]
pub type RD_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN3`"]
pub struct RD_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN3_W<'a> {
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
#[doc = "Reader of field `PROG_EN3`"]
pub type PROG_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN3`"]
pub struct PROG_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN3_W<'a> {
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
#[doc = "Reader of field `ERASE_EN3`"]
pub type ERASE_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN3`"]
pub struct ERASE_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN3_W<'a> {
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
#[doc = "Reader of field `BASE3`"]
pub type BASE3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE3`"]
pub struct BASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE3`"]
pub type SIZE3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE3`"]
pub struct SIZE3_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL3"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL3"]
    #[inline(always)]
    pub fn rd_en3(&self) -> RD_EN3_R {
        RD_EN3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL3"]
    #[inline(always)]
    pub fn prog_en3(&self) -> PROG_EN3_R {
        PROG_EN3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL3"]
    #[inline(always)]
    pub fn erase_en3(&self) -> ERASE_EN3_R {
        ERASE_EN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL3"]
    #[inline(always)]
    pub fn base3(&self) -> BASE3_R {
        BASE3_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL3"]
    #[inline(always)]
    pub fn size3(&self) -> SIZE3_R {
        SIZE3_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL3"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W {
        EN3_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL3"]
    #[inline(always)]
    pub fn rd_en3(&mut self) -> RD_EN3_W {
        RD_EN3_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL3"]
    #[inline(always)]
    pub fn prog_en3(&mut self) -> PROG_EN3_W {
        PROG_EN3_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL3"]
    #[inline(always)]
    pub fn erase_en3(&mut self) -> ERASE_EN3_W {
        ERASE_EN3_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL3"]
    #[inline(always)]
    pub fn base3(&mut self) -> BASE3_W {
        BASE3_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL3"]
    #[inline(always)]
    pub fn size3(&mut self) -> SIZE3_W {
        SIZE3_W { w: self }
    }
}
