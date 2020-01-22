#[doc = "Reader of register FIFO_LEVEL"]
pub type R = crate::R<u32, super::FIFO_LEVEL>;
#[doc = "Writer for register FIFO_LEVEL"]
pub type W = crate::W<u32, super::FIFO_LEVEL>;
#[doc = "Register FIFO_LEVEL `reset()`'s with value 0x80"]
impl crate::ResetValue for super::FIFO_LEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `rxlvl`"]
pub type RXLVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rxlvl`"]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `txlvl`"]
pub type TXLVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `txlvl`"]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RX FIFO level. If RX SRAM FIFO level exceeds this value, it triggers interrupt."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TX FIFO level. If TX SRAM FIFO level drops below this value, it triggers interrupt."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RX FIFO level. If RX SRAM FIFO level exceeds this value, it triggers interrupt."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
    #[doc = "Bits 16:31 - TX FIFO level. If TX SRAM FIFO level drops below this value, it triggers interrupt."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
}
