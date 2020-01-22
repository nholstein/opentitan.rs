#[doc = "Reader of register REGION_CFG_REGWEN"]
pub type R = crate::R<u32, super::REGION_CFG_REGWEN>;
#[doc = "Writer for register REGION_CFG_REGWEN"]
pub type W = crate::W<u32, super::REGION_CFG_REGWEN>;
#[doc = "Register REGION_CFG_REGWEN `reset()`'s with value 0xff"]
impl crate::ResetValue for super::REGION_CFG_REGWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `REGION0`"]
pub type REGION0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION0`"]
pub struct REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0_W<'a> {
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
#[doc = "Reader of field `REGION1`"]
pub type REGION1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION1`"]
pub struct REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_W<'a> {
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
#[doc = "Reader of field `REGION2`"]
pub type REGION2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION2`"]
pub struct REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2_W<'a> {
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
#[doc = "Reader of field `REGION3`"]
pub type REGION3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION3`"]
pub struct REGION3_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3_W<'a> {
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
#[doc = "Reader of field `REGION4`"]
pub type REGION4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION4`"]
pub struct REGION4_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `REGION5`"]
pub type REGION5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION5`"]
pub struct REGION5_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `REGION6`"]
pub type REGION6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION6`"]
pub struct REGION6_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `REGION7`"]
pub type REGION7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION7`"]
pub struct REGION7_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region register write enable. Once set to 0, it can longer be configured to 1 for FLASH_CTRL0"]
    #[inline(always)]
    pub fn region0(&self) -> REGION0_R {
        REGION0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for FLASH_CTRL1"]
    #[inline(always)]
    pub fn region1(&self) -> REGION1_R {
        REGION1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for FLASH_CTRL2"]
    #[inline(always)]
    pub fn region2(&self) -> REGION2_R {
        REGION2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for FLASH_CTRL3"]
    #[inline(always)]
    pub fn region3(&self) -> REGION3_R {
        REGION3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for FLASH_CTRL4"]
    #[inline(always)]
    pub fn region4(&self) -> REGION4_R {
        REGION4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for FLASH_CTRL5"]
    #[inline(always)]
    pub fn region5(&self) -> REGION5_R {
        REGION5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for FLASH_CTRL6"]
    #[inline(always)]
    pub fn region6(&self) -> REGION6_R {
        REGION6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for FLASH_CTRL7"]
    #[inline(always)]
    pub fn region7(&self) -> REGION7_R {
        REGION7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region register write enable. Once set to 0, it can longer be configured to 1 for FLASH_CTRL0"]
    #[inline(always)]
    pub fn region0(&mut self) -> REGION0_W {
        REGION0_W { w: self }
    }
    #[doc = "Bit 1 - for FLASH_CTRL1"]
    #[inline(always)]
    pub fn region1(&mut self) -> REGION1_W {
        REGION1_W { w: self }
    }
    #[doc = "Bit 2 - for FLASH_CTRL2"]
    #[inline(always)]
    pub fn region2(&mut self) -> REGION2_W {
        REGION2_W { w: self }
    }
    #[doc = "Bit 3 - for FLASH_CTRL3"]
    #[inline(always)]
    pub fn region3(&mut self) -> REGION3_W {
        REGION3_W { w: self }
    }
    #[doc = "Bit 4 - for FLASH_CTRL4"]
    #[inline(always)]
    pub fn region4(&mut self) -> REGION4_W {
        REGION4_W { w: self }
    }
    #[doc = "Bit 5 - for FLASH_CTRL5"]
    #[inline(always)]
    pub fn region5(&mut self) -> REGION5_W {
        REGION5_W { w: self }
    }
    #[doc = "Bit 6 - for FLASH_CTRL6"]
    #[inline(always)]
    pub fn region6(&mut self) -> REGION6_W {
        REGION6_W { w: self }
    }
    #[doc = "Bit 7 - for FLASH_CTRL7"]
    #[inline(always)]
    pub fn region7(&mut self) -> REGION7_W {
        REGION7_W { w: self }
    }
}
