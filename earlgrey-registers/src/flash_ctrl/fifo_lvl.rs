#[doc = "Reader of register FIFO_LVL"]
pub type R = crate::R<u32, super::FIFO_LVL>;
#[doc = "Writer for register FIFO_LVL"]
pub type W = crate::W<u32, super::FIFO_LVL>;
#[doc = "Register FIFO_LVL `reset()`'s with value 0x0f0f"]
impl crate::ResetValue for super::FIFO_LVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f0f
    }
}
#[doc = "Reader of field `PROG`"]
pub type PROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROG`"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - When the program fifo drains to this level, trigger an interrupt. Default value is set such that interrupt does not trigger at reset."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - When the read fifo fills to this level, trigger an interrupt. Default value is set such that interrupt does not trigger at reset."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - When the program fifo drains to this level, trigger an interrupt. Default value is set such that interrupt does not trigger at reset."]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bits 8:12 - When the read fifo fills to this level, trigger an interrupt. Default value is set such that interrupt does not trigger at reset."]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
}
