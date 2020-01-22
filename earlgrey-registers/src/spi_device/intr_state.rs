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
#[doc = "Reader of field `rxf`"]
pub type RXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxf`"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
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
#[doc = "Reader of field `rxlvl`"]
pub type RXLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxlvl`"]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
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
#[doc = "Reader of field `txlvl`"]
pub type TXLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txlvl`"]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
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
#[doc = "Reader of field `rxerr`"]
pub type RXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxerr`"]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
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
#[doc = "Reader of field `rxoverflow`"]
pub type RXOVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxoverflow`"]
pub struct RXOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERFLOW_W<'a> {
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
#[doc = "Reader of field `txunderflow`"]
pub type TXUNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txunderflow`"]
pub struct TXUNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERFLOW_W<'a> {
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
    #[doc = "Bit 0 - RX SRAM FIFO Full"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX SRAM FIFO is above the level"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX SRAM FIFO is under the level"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MOSI in FwMode has error"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Async FIFO overflow"]
    #[inline(always)]
    pub fn rxoverflow(&self) -> RXOVERFLOW_R {
        RXOVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Async FIFO underflow"]
    #[inline(always)]
    pub fn txunderflow(&self) -> TXUNDERFLOW_R {
        TXUNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX SRAM FIFO Full"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 1 - RX SRAM FIFO is above the level"]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
    #[doc = "Bit 2 - TX SRAM FIFO is under the level"]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    #[doc = "Bit 3 - MOSI in FwMode has error"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    #[doc = "Bit 4 - RX Async FIFO overflow"]
    #[inline(always)]
    pub fn rxoverflow(&mut self) -> RXOVERFLOW_W {
        RXOVERFLOW_W { w: self }
    }
    #[doc = "Bit 5 - TX Async FIFO underflow"]
    #[inline(always)]
    pub fn txunderflow(&mut self) -> TXUNDERFLOW_W {
        TXUNDERFLOW_W { w: self }
    }
}
