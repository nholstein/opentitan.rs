#[doc = "Writer for register INTR_TEST"]
pub type W = crate::W<u32, super::INTR_TEST>;
#[doc = "Register INTR_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Write 1 to force !!INTR_STATE.prog_empty to 1"]
    #[inline(always)]
    pub fn prog_empty(&mut self) -> PROG_EMPTY_W {
        PROG_EMPTY_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to force !!INTR_STATE.prog_lvl to 1"]
    #[inline(always)]
    pub fn prog_lvl(&mut self) -> PROG_LVL_W {
        PROG_LVL_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 to force !!INTR_STATE.rd_full to 1"]
    #[inline(always)]
    pub fn rd_full(&mut self) -> RD_FULL_W {
        RD_FULL_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 to force !!INTR_STATE.rd_lvl to 1"]
    #[inline(always)]
    pub fn rd_lvl(&mut self) -> RD_LVL_W {
        RD_LVL_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 to force !!INTR_STATE.op_done to 1"]
    #[inline(always)]
    pub fn op_done(&mut self) -> OP_DONE_W {
        OP_DONE_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 to force !!INTR_STATE.op_error to 1"]
    #[inline(always)]
    pub fn op_error(&mut self) -> OP_ERROR_W {
        OP_ERROR_W { w: self }
    }
}
