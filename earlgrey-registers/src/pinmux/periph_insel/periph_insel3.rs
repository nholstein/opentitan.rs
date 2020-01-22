#[doc = "Reader of register PERIPH_INSEL3"]
pub type R = crate::R<u32, super::PERIPH_INSEL3>;
#[doc = "Writer for register PERIPH_INSEL3"]
pub type W = crate::W<u32, super::PERIPH_INSEL3>;
#[doc = "Register PERIPH_INSEL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN15`"]
pub type IN15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN15`"]
pub struct IN15_W<'a> {
    w: &'a mut W,
}
impl<'a> IN15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN16`"]
pub type IN16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN16`"]
pub struct IN16_W<'a> {
    w: &'a mut W,
}
impl<'a> IN16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN17`"]
pub type IN17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN17`"]
pub struct IN17_W<'a> {
    w: &'a mut W,
}
impl<'a> IN17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN18`"]
pub type IN18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN18`"]
pub struct IN18_W<'a> {
    w: &'a mut W,
}
impl<'a> IN18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN19`"]
pub type IN19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN19`"]
pub struct IN19_W<'a> {
    w: &'a mut W,
}
impl<'a> IN19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN15"]
    #[inline(always)]
    pub fn in15(&self) -> IN15_R {
        IN15_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN16"]
    #[inline(always)]
    pub fn in16(&self) -> IN16_R {
        IN16_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN17"]
    #[inline(always)]
    pub fn in17(&self) -> IN17_R {
        IN17_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN18"]
    #[inline(always)]
    pub fn in18(&self) -> IN18_R {
        IN18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN19"]
    #[inline(always)]
    pub fn in19(&self) -> IN19_R {
        IN19_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN15"]
    #[inline(always)]
    pub fn in15(&mut self) -> IN15_W {
        IN15_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN16"]
    #[inline(always)]
    pub fn in16(&mut self) -> IN16_W {
        IN16_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN17"]
    #[inline(always)]
    pub fn in17(&mut self) -> IN17_W {
        IN17_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN18"]
    #[inline(always)]
    pub fn in18(&mut self) -> IN18_W {
        IN18_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN19"]
    #[inline(always)]
    pub fn in19(&mut self) -> IN19_W {
        IN19_W { w: self }
    }
}
