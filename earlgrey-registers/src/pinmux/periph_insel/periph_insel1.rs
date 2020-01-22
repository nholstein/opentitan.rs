#[doc = "Reader of register PERIPH_INSEL1"]
pub type R = crate::R<u32, super::PERIPH_INSEL1>;
#[doc = "Writer for register PERIPH_INSEL1"]
pub type W = crate::W<u32, super::PERIPH_INSEL1>;
#[doc = "Register PERIPH_INSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN5`"]
pub struct IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN6`"]
pub struct IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN7`"]
pub struct IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN8`"]
pub type IN8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN8`"]
pub struct IN8_W<'a> {
    w: &'a mut W,
}
impl<'a> IN8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN9`"]
pub type IN9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN9`"]
pub struct IN9_W<'a> {
    w: &'a mut W,
}
impl<'a> IN9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN8"]
    #[inline(always)]
    pub fn in8(&self) -> IN8_R {
        IN8_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN9"]
    #[inline(always)]
    pub fn in9(&self) -> IN9_R {
        IN9_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN5"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W {
        IN5_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN6"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W {
        IN6_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN7"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W {
        IN7_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN8"]
    #[inline(always)]
    pub fn in8(&mut self) -> IN8_W {
        IN8_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN9"]
    #[inline(always)]
    pub fn in9(&mut self) -> IN9_W {
        IN9_W { w: self }
    }
}
