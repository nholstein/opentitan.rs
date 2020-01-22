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
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `OP`"]
pub type OP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OP`"]
pub struct OP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ERASE_SEL`"]
pub type ERASE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE_SEL`"]
pub struct ERASE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_SEL_W<'a> {
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
#[doc = "Reader of field `FIFO_RST`"]
pub type FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_RST`"]
pub struct FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RST_W<'a> {
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
#[doc = "Reader of field `NUM`"]
pub type NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NUM`"]
pub struct NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start flash transaction. This bit shall only be set after the other fields of the CONTROL register and ADDR have been programmed"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Flash operation selection"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Flash operation selection"]
    #[inline(always)]
    pub fn erase_sel(&self) -> ERASE_SEL_R {
        ERASE_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RST FIFOs"]
    #[inline(always)]
    pub fn fifo_rst(&self) -> FIFO_RST_R {
        FIFO_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Number of flash words the flash operation should read or program."]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Start flash transaction. This bit shall only be set after the other fields of the CONTROL register and ADDR have been programmed"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 4:5 - Flash operation selection"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W {
        OP_W { w: self }
    }
    #[doc = "Bit 6 - Flash operation selection"]
    #[inline(always)]
    pub fn erase_sel(&mut self) -> ERASE_SEL_W {
        ERASE_SEL_W { w: self }
    }
    #[doc = "Bit 7 - RST FIFOs"]
    #[inline(always)]
    pub fn fifo_rst(&mut self) -> FIFO_RST_W {
        FIFO_RST_W { w: self }
    }
    #[doc = "Bits 16:27 - Number of flash words the flash operation should read or program."]
    #[inline(always)]
    pub fn num(&mut self) -> NUM_W {
        NUM_W { w: self }
    }
}
