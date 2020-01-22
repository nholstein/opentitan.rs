#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT`"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `rst_txfifo`"]
pub type RST_TXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rst_txfifo`"]
pub struct RST_TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_TXFIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `rst_rxfifo`"]
pub type RST_RXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rst_rxfifo`"]
pub struct RST_RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_RXFIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Abort pending jobs. If TX_FIFO (async) is full, this command can let TXF Control logic back to Idle state"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SPI Device operation mode. Currently only FwMode is supported."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Reset Async TX_FIFO. This only resets asynchronous fifo. If firmware wants to reset SRAM FIFO, it should write 0 into read/write pointers. _Note_: This value should be controlled only when SPI interface is in Idle state as this reset signal doesn't have reset synchronizer."]
    #[inline(always)]
    pub fn rst_txfifo(&self) -> RST_TXFIFO_R {
        RST_TXFIFO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset Async RX_FIFO. This only resets asynchronous fifo. If firmware wants to reset SRAM FIFO, it should write 0 into read pointer and write pointer. _Note_: This value should be controlled only when SPI interface is in Idle state as this reset signal doesn't have reset synchronizer."]
    #[inline(always)]
    pub fn rst_rxfifo(&self) -> RST_RXFIFO_R {
        RST_RXFIFO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Abort pending jobs. If TX_FIFO (async) is full, this command can let TXF Control logic back to Idle state"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI Device operation mode. Currently only FwMode is supported."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 16 - Reset Async TX_FIFO. This only resets asynchronous fifo. If firmware wants to reset SRAM FIFO, it should write 0 into read/write pointers. _Note_: This value should be controlled only when SPI interface is in Idle state as this reset signal doesn't have reset synchronizer."]
    #[inline(always)]
    pub fn rst_txfifo(&mut self) -> RST_TXFIFO_W {
        RST_TXFIFO_W { w: self }
    }
    #[doc = "Bit 17 - Reset Async RX_FIFO. This only resets asynchronous fifo. If firmware wants to reset SRAM FIFO, it should write 0 into read pointer and write pointer. _Note_: This value should be controlled only when SPI interface is in Idle state as this reset signal doesn't have reset synchronizer."]
    #[inline(always)]
    pub fn rst_rxfifo(&mut self) -> RST_RXFIFO_W {
        RST_RXFIFO_W { w: self }
    }
}
