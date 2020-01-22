#[doc = "Reader of register FIFO_CTRL"]
pub type R = crate::R<u32, super::FIFO_CTRL>;
#[doc = "Writer for register FIFO_CTRL"]
pub type W = crate::W<u32, super::FIFO_CTRL>;
#[doc = "Register FIFO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXRST`"]
pub struct RXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRST_W<'a> {
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
#[doc = "Write proxy for field `TXRST`"]
pub struct TXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRST_W<'a> {
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
#[doc = "Reader of field `RXILVL`"]
pub type RXILVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXILVL`"]
pub struct RXILVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXILVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXILVL`"]
pub type TXILVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXILVL`"]
pub struct TXILVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXILVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:4 - Trigger level for RX interrupts. If the FIFO depth is greater than or equal to the setting, it raises rx_watermark interrupt."]
    #[inline(always)]
    pub fn rxilvl(&self) -> RXILVL_R {
        RXILVL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:6 - Trigger level for TX interrupts. If the FIFO depth is greater than or equal to the setting, it raises tx_watermark interrupt."]
    #[inline(always)]
    pub fn txilvl(&self) -> TXILVL_R {
        TXILVL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX fifo reset. Write 1 to the register resets RX_FIFO. Read returns 0"]
    #[inline(always)]
    pub fn rxrst(&mut self) -> RXRST_W {
        RXRST_W { w: self }
    }
    #[doc = "Bit 1 - TX fifo reset. Write 1 to the register resets TX_FIFO. Read returns 0"]
    #[inline(always)]
    pub fn txrst(&mut self) -> TXRST_W {
        TXRST_W { w: self }
    }
    #[doc = "Bits 2:4 - Trigger level for RX interrupts. If the FIFO depth is greater than or equal to the setting, it raises rx_watermark interrupt."]
    #[inline(always)]
    pub fn rxilvl(&mut self) -> RXILVL_W {
        RXILVL_W { w: self }
    }
    #[doc = "Bits 5:6 - Trigger level for TX interrupts. If the FIFO depth is greater than or equal to the setting, it raises tx_watermark interrupt."]
    #[inline(always)]
    pub fn txilvl(&mut self) -> TXILVL_W {
        TXILVL_W { w: self }
    }
}
