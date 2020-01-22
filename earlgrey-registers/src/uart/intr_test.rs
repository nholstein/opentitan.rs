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
#[doc = "Write proxy for field `tx_watermark`"]
pub struct TX_WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WATERMARK_W<'a> {
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
#[doc = "Write proxy for field `rx_watermark`"]
pub struct RX_WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WATERMARK_W<'a> {
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
#[doc = "Write proxy for field `tx_empty`"]
pub struct TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_W<'a> {
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
#[doc = "Write proxy for field `rx_overflow`"]
pub struct RX_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERFLOW_W<'a> {
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
#[doc = "Write proxy for field `rx_frame_err`"]
pub struct RX_FRAME_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAME_ERR_W<'a> {
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
#[doc = "Write proxy for field `rx_break_err`"]
pub struct RX_BREAK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BREAK_ERR_W<'a> {
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
#[doc = "Write proxy for field `rx_timeout`"]
pub struct RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_W<'a> {
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
#[doc = "Write proxy for field `rx_parity_err`"]
pub struct RX_PARITY_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PARITY_ERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Write 1 to force !!INTR_STATE.tx_watermark to 1"]
    #[inline(always)]
    pub fn tx_watermark(&mut self) -> TX_WATERMARK_W {
        TX_WATERMARK_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to force !!INTR_STATE.rx_watermark to 1"]
    #[inline(always)]
    pub fn rx_watermark(&mut self) -> RX_WATERMARK_W {
        RX_WATERMARK_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 to force !!INTR_STATE.tx_empty to 1"]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 to force !!INTR_STATE.rx_overflow to 1"]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W {
        RX_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 to force !!INTR_STATE.rx_frame_err to 1"]
    #[inline(always)]
    pub fn rx_frame_err(&mut self) -> RX_FRAME_ERR_W {
        RX_FRAME_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 to force !!INTR_STATE.rx_break_err to 1"]
    #[inline(always)]
    pub fn rx_break_err(&mut self) -> RX_BREAK_ERR_W {
        RX_BREAK_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Write 1 to force !!INTR_STATE.rx_timeout to 1"]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W {
        RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 7 - Write 1 to force !!INTR_STATE.rx_parity_err to 1"]
    #[inline(always)]
    pub fn rx_parity_err(&mut self) -> RX_PARITY_ERR_W {
        RX_PARITY_ERR_W { w: self }
    }
}
