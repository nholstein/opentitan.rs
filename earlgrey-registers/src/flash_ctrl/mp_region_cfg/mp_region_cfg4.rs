#[doc = "Reader of register MP_REGION_CFG4"]
pub type R = crate::R<u32, super::MP_REGION_CFG4>;
#[doc = "Writer for register MP_REGION_CFG4"]
pub type W = crate::W<u32, super::MP_REGION_CFG4>;
#[doc = "Register MP_REGION_CFG4 `reset()`'s with value 0"]
impl crate::ResetValue for super::MP_REGION_CFG4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN4`"]
pub type EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN4`"]
pub struct EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EN4_W<'a> {
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
#[doc = "Reader of field `RD_EN4`"]
pub type RD_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN4`"]
pub struct RD_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN4_W<'a> {
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
#[doc = "Reader of field `PROG_EN4`"]
pub type PROG_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN4`"]
pub struct PROG_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN4_W<'a> {
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
#[doc = "Reader of field `ERASE_EN4`"]
pub type ERASE_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN4`"]
pub struct ERASE_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN4_W<'a> {
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
#[doc = "Reader of field `BASE4`"]
pub type BASE4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE4`"]
pub struct BASE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE4`"]
pub type SIZE4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE4`"]
pub struct SIZE4_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL4"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL4"]
    #[inline(always)]
    pub fn rd_en4(&self) -> RD_EN4_R {
        RD_EN4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL4"]
    #[inline(always)]
    pub fn prog_en4(&self) -> PROG_EN4_R {
        PROG_EN4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL4"]
    #[inline(always)]
    pub fn erase_en4(&self) -> ERASE_EN4_R {
        ERASE_EN4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL4"]
    #[inline(always)]
    pub fn base4(&self) -> BASE4_R {
        BASE4_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL4"]
    #[inline(always)]
    pub fn size4(&self) -> SIZE4_R {
        SIZE4_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Region enabled, following fields apply for FLASH_CTRL4"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W {
        EN4_W { w: self }
    }
    #[doc = "Bit 1 - Region can be read for FLASH_CTRL4"]
    #[inline(always)]
    pub fn rd_en4(&mut self) -> RD_EN4_W {
        RD_EN4_W { w: self }
    }
    #[doc = "Bit 2 - Region can be programmed for FLASH_CTRL4"]
    #[inline(always)]
    pub fn prog_en4(&mut self) -> PROG_EN4_W {
        PROG_EN4_W { w: self }
    }
    #[doc = "Bit 3 - Region can be erased for FLASH_CTRL4"]
    #[inline(always)]
    pub fn erase_en4(&mut self) -> ERASE_EN4_W {
        ERASE_EN4_W { w: self }
    }
    #[doc = "Bits 4:12 - Region base page. Note the granularity is page, not byte or word for FLASH_CTRL4"]
    #[inline(always)]
    pub fn base4(&mut self) -> BASE4_W {
        BASE4_W { w: self }
    }
    #[doc = "Bits 16:24 - Region size in number of pages for FLASH_CTRL4"]
    #[inline(always)]
    pub fn size4(&mut self) -> SIZE4_W {
        SIZE4_W { w: self }
    }
}
