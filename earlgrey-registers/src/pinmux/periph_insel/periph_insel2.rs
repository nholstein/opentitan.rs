#[doc = "Reader of register PERIPH_INSEL2"]
pub type R = crate::R<u32, super::PERIPH_INSEL2>;
#[doc = "Writer for register PERIPH_INSEL2"]
pub type W = crate::W<u32, super::PERIPH_INSEL2>;
#[doc = "Register PERIPH_INSEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN10`"]
pub type IN10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN10`"]
pub struct IN10_W<'a> {
    w: &'a mut W,
}
impl<'a> IN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN11`"]
pub type IN11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN11`"]
pub struct IN11_W<'a> {
    w: &'a mut W,
}
impl<'a> IN11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN12`"]
pub type IN12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN12`"]
pub struct IN12_W<'a> {
    w: &'a mut W,
}
impl<'a> IN12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN13`"]
pub type IN13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN13`"]
pub struct IN13_W<'a> {
    w: &'a mut W,
}
impl<'a> IN13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN14`"]
pub type IN14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN14`"]
pub struct IN14_W<'a> {
    w: &'a mut W,
}
impl<'a> IN14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN10"]
    #[inline(always)]
    pub fn in10(&self) -> IN10_R {
        IN10_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN11"]
    #[inline(always)]
    pub fn in11(&self) -> IN11_R {
        IN11_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN12"]
    #[inline(always)]
    pub fn in12(&self) -> IN12_R {
        IN12_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN13"]
    #[inline(always)]
    pub fn in13(&self) -> IN13_R {
        IN13_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN14"]
    #[inline(always)]
    pub fn in14(&self) -> IN14_R {
        IN14_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN10"]
    #[inline(always)]
    pub fn in10(&mut self) -> IN10_W {
        IN10_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN11"]
    #[inline(always)]
    pub fn in11(&mut self) -> IN11_W {
        IN11_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN12"]
    #[inline(always)]
    pub fn in12(&mut self) -> IN12_W {
        IN12_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN13"]
    #[inline(always)]
    pub fn in13(&mut self) -> IN13_W {
        IN13_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN14"]
    #[inline(always)]
    pub fn in14(&mut self) -> IN14_W {
        IN14_W { w: self }
    }
}
