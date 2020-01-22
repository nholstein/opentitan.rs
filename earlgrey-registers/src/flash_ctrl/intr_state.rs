#[doc = "Reader of register INTR_STATE"]
pub type R = crate::R<u32, super::INTR_STATE>;
#[doc = "Writer for register INTR_STATE"]
pub type W = crate::W<u32, super::INTR_STATE>;
#[doc = "Register INTR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `prog_empty`"]
pub type PROG_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `prog_empty`"]
pub struct PROG_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_EMPTY_W<'a> {
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
#[doc = "Reader of field `prog_lvl`"]
pub type PROG_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `prog_lvl`"]
pub struct PROG_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_LVL_W<'a> {
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
#[doc = "Reader of field `rd_full`"]
pub type RD_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_full`"]
pub struct RD_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_FULL_W<'a> {
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
#[doc = "Reader of field `rd_lvl`"]
pub type RD_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lvl`"]
pub struct RD_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LVL_W<'a> {
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
#[doc = "Reader of field `op_done`"]
pub type OP_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `op_done`"]
pub struct OP_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_DONE_W<'a> {
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
#[doc = "Reader of field `op_error`"]
pub type OP_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `op_error`"]
pub struct OP_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_ERROR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Program FIFO empty"]
    #[inline(always)]
    pub fn prog_empty(&self) -> PROG_EMPTY_R {
        PROG_EMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Program FIFO drained to level"]
    #[inline(always)]
    pub fn prog_lvl(&self) -> PROG_LVL_R {
        PROG_LVL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read FIFO full"]
    #[inline(always)]
    pub fn rd_full(&self) -> RD_FULL_R {
        RD_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read FIFO filled to level"]
    #[inline(always)]
    pub fn rd_lvl(&self) -> RD_LVL_R {
        RD_LVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Operation complete"]
    #[inline(always)]
    pub fn op_done(&self) -> OP_DONE_R {
        OP_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Operation failed with error"]
    #[inline(always)]
    pub fn op_error(&self) -> OP_ERROR_R {
        OP_ERROR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Program FIFO empty"]
    #[inline(always)]
    pub fn prog_empty(&mut self) -> PROG_EMPTY_W {
        PROG_EMPTY_W { w: self }
    }
    #[doc = "Bit 1 - Program FIFO drained to level"]
    #[inline(always)]
    pub fn prog_lvl(&mut self) -> PROG_LVL_W {
        PROG_LVL_W { w: self }
    }
    #[doc = "Bit 2 - Read FIFO full"]
    #[inline(always)]
    pub fn rd_full(&mut self) -> RD_FULL_W {
        RD_FULL_W { w: self }
    }
    #[doc = "Bit 3 - Read FIFO filled to level"]
    #[inline(always)]
    pub fn rd_lvl(&mut self) -> RD_LVL_W {
        RD_LVL_W { w: self }
    }
    #[doc = "Bit 4 - Operation complete"]
    #[inline(always)]
    pub fn op_done(&mut self) -> OP_DONE_W {
        OP_DONE_W { w: self }
    }
    #[doc = "Bit 5 - Operation failed with error"]
    #[inline(always)]
    pub fn op_error(&mut self) -> OP_ERROR_W {
        OP_ERROR_W { w: self }
    }
}
