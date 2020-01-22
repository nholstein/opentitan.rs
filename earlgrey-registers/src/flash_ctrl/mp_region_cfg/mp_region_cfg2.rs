#[doc = "Reader of register MP_REGION_CFG2"]
pub type R = crate::R<u32, super::MP_REGION_CFG2>;
#[doc = "Writer for register MP_REGION_CFG2"]
pub type W = crate::W<u32, super::MP_REGION_CFG2>;
#[doc = "Register MP_REGION_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN2`"]
pub type EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN2`"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
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
#[doc = "Reader of field `RD_EN2`"]
pub type RD_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN2`"]
pub struct RD_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN2_W<'a> {
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
#[doc = "Reader of field `PROG_EN2`"]
pub type PROG_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN2`"]
pub struct PROG_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN2_W<'a> {
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
#[doc = "Reader of field `ERASE_EN2`"]
pub type ERASE_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN2`"]
pub struct ERASE_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN2_W<'a> {
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
#[doc = "Reader of field `BASE2`"]
pub type BASE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE2`"]
pub struct BASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE2`"]
pub type SIZE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE2`"]
pub struct SIZE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL2"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL2"]
    #[inline(always)]
    pub fn rd_en2(&self) -> RD_EN2_R {
        RD_EN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL2"]
    #[inline(always)]
    pub fn prog_en2(&self) -> PROG_EN2_R {
        PROG_EN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL2"]
    #[inline(always)]
    pub fn erase_en2(&self) -> ERASE_EN2_R {
        ERASE_EN2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL2"]
    #[inline(always)]
    pub fn base2(&self) -> BASE2_R {
        BASE2_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL2"]
    #[inline(always)]
    pub fn size2(&self) -> SIZE2_R {
        SIZE2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL2"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL2"]
    #[inline(always)]
    pub fn rd_en2(&mut self) -> RD_EN2_W {
        RD_EN2_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL2"]
    #[inline(always)]
    pub fn prog_en2(&mut self) -> PROG_EN2_W {
        PROG_EN2_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL2"]
    #[inline(always)]
    pub fn erase_en2(&mut self) -> ERASE_EN2_W {
        ERASE_EN2_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL2"]
    #[inline(always)]
    pub fn base2(&mut self) -> BASE2_W {
        BASE2_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL2"]
    #[inline(always)]
    pub fn size2(&mut self) -> SIZE2_W {
        SIZE2_W { w: self }
    }
}
