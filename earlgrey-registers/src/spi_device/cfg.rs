#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x7f00"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f00
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
#[doc = "Reader of field `tx_order`"]
pub type TX_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_order`"]
pub struct TX_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ORDER_W<'a> {
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
#[doc = "Reader of field `rx_order`"]
pub type RX_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_order`"]
pub struct RX_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ORDER_W<'a> {
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
#[doc = "Reader of field `timer_v`"]
pub type TIMER_V_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `timer_v`"]
pub struct TIMER_V_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_V_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock polarity. 0 for normal SPI, 1 for negative edge latch"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data phase. 0 for negative edge change, 1 for positive edge change"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX bit order on MISO. 0 for MSB to LSB, 1 for LSB to MSB"]
    #[inline(always)]
    pub fn tx_order(&self) -> TX_ORDER_R {
        TX_ORDER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX bit order on MOSI. Module stores bitstream from MSB to LSB if value is 0."]
    #[inline(always)]
    pub fn rx_order(&self) -> RX_ORDER_R {
        RX_ORDER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - number of clocks for RXF to wait. To reduce traffic to SRAM, RXF control module waits given clock cycle if it doesn't fill SRAM data width even if Async RX FIFO is empty."]
    #[inline(always)]
    pub fn timer_v(&self) -> TIMER_V_R {
        TIMER_V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock polarity. 0 for normal SPI, 1 for negative edge latch"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 1 - Data phase. 0 for negative edge change, 1 for positive edge change"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 2 - TX bit order on MISO. 0 for MSB to LSB, 1 for LSB to MSB"]
    #[inline(always)]
    pub fn tx_order(&mut self) -> TX_ORDER_W {
        TX_ORDER_W { w: self }
    }
    #[doc = "Bit 3 - RX bit order on MOSI. Module stores bitstream from MSB to LSB if value is 0."]
    #[inline(always)]
    pub fn rx_order(&mut self) -> RX_ORDER_W {
        RX_ORDER_W { w: self }
    }
    #[doc = "Bits 8:15 - number of clocks for RXF to wait. To reduce traffic to SRAM, RXF control module waits given clock cycle if it doesn't fill SRAM data width even if Async RX FIFO is empty."]
    #[inline(always)]
    pub fn timer_v(&mut self) -> TIMER_V_W {
        TIMER_V_W { w: self }
    }
}
