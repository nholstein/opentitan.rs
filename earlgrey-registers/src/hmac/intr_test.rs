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
#[doc = "Write proxy for field `hmac_done`"]
pub struct HMAC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> HMAC_DONE_W<'a> {
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
#[doc = "Write proxy for field `fifo_full`"]
pub struct FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FULL_W<'a> {
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
#[doc = "Write proxy for field `hmac_err`"]
pub struct HMAC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HMAC_ERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Write 1 to force !!INTR_STATE.hmac_done to 1"]
    #[inline(always)]
    pub fn hmac_done(&mut self) -> HMAC_DONE_W {
        HMAC_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to force !!INTR_STATE.fifo_full to 1"]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W {
        FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 to force !!INTR_STATE.hmac_err to 1"]
    #[inline(always)]
    pub fn hmac_err(&mut self) -> HMAC_ERR_W {
        HMAC_ERR_W { w: self }
    }
}
