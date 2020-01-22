#[doc = "Reader of register DEFAULT_REGION"]
pub type R = crate::R<u32, super::DEFAULT_REGION>;
#[doc = "Writer for register DEFAULT_REGION"]
pub type W = crate::W<u32, super::DEFAULT_REGION>;
#[doc = "Register DEFAULT_REGION `reset()`'s with value 0"]
impl crate::ResetValue for super::DEFAULT_REGION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD_EN`"]
pub type RD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_EN`"]
pub struct RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN_W<'a> {
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
#[doc = "Reader of field `PROG_EN`"]
pub type PROG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_EN`"]
pub struct PROG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EN_W<'a> {
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
#[doc = "Reader of field `ERASE_EN`"]
pub type ERASE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_EN`"]
pub struct ERASE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Region can be read"]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Region can be programmed"]
    #[inline(always)]
    pub fn prog_en(&self) -> PROG_EN_R {
        PROG_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Region can be erased"]
    #[inline(always)]
    pub fn erase_en(&self) -> ERASE_EN_R {
        ERASE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region can be read"]
    #[inline(always)]
    pub fn rd_en(&mut self) -> RD_EN_W {
        RD_EN_W { w: self }
    }
    #[doc = "Bit 1 - Region can be programmed"]
    #[inline(always)]
    pub fn prog_en(&mut self) -> PROG_EN_W {
        PROG_EN_W { w: self }
    }
    #[doc = "Bit 2 - Region can be erased"]
    #[inline(always)]
    pub fn erase_en(&mut self) -> ERASE_EN_W {
        ERASE_EN_W { w: self }
    }
}
