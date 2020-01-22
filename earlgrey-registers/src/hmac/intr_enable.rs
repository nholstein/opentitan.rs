#[doc = "Reader of register INTR_ENABLE"]
pub type R = crate::R<u32, super::INTR_ENABLE>;
#[doc = "Writer for register INTR_ENABLE"]
pub type W = crate::W<u32, super::INTR_ENABLE>;
#[doc = "Register INTR_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hmac_done`"]
pub type HMAC_DONE_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `fifo_full`"]
pub type FIFO_FULL_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `hmac_err`"]
pub type HMAC_ERR_R = crate::R<bool, bool>;
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
impl R {
    #[doc = "Bit 0 - Enable interrupt when !!INTR_STATE.hmac_done is set"]
    #[inline(always)]
    pub fn hmac_done(&self) -> HMAC_DONE_R {
        HMAC_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt when !!INTR_STATE.fifo_full is set"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt when !!INTR_STATE.hmac_err is set"]
    #[inline(always)]
    pub fn hmac_err(&self) -> HMAC_ERR_R {
        HMAC_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt when !!INTR_STATE.hmac_done is set"]
    #[inline(always)]
    pub fn hmac_done(&mut self) -> HMAC_DONE_W {
        HMAC_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt when !!INTR_STATE.fifo_full is set"]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W {
        FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt when !!INTR_STATE.hmac_err is set"]
    #[inline(always)]
    pub fn hmac_err(&mut self) -> HMAC_ERR_W {
        HMAC_ERR_W { w: self }
    }
}
