#[doc = "Reader of register PERIPH_INSEL5"]
pub type R = crate::R<u32, super::PERIPH_INSEL5>;
#[doc = "Writer for register PERIPH_INSEL5"]
pub type W = crate::W<u32, super::PERIPH_INSEL5>;
#[doc = "Register PERIPH_INSEL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN25`"]
pub type IN25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN25`"]
pub struct IN25_W<'a> {
    w: &'a mut W,
}
impl<'a> IN25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN26`"]
pub type IN26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN26`"]
pub struct IN26_W<'a> {
    w: &'a mut W,
}
impl<'a> IN26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN27`"]
pub type IN27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN27`"]
pub struct IN27_W<'a> {
    w: &'a mut W,
}
impl<'a> IN27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN28`"]
pub type IN28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN28`"]
pub struct IN28_W<'a> {
    w: &'a mut W,
}
impl<'a> IN28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN29`"]
pub type IN29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN29`"]
pub struct IN29_W<'a> {
    w: &'a mut W,
}
impl<'a> IN29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN25"]
    #[inline(always)]
    pub fn in25(&self) -> IN25_R {
        IN25_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN26"]
    #[inline(always)]
    pub fn in26(&self) -> IN26_R {
        IN26_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN27"]
    #[inline(always)]
    pub fn in27(&self) -> IN27_R {
        IN27_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN28"]
    #[inline(always)]
    pub fn in28(&self) -> IN28_R {
        IN28_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN29"]
    #[inline(always)]
    pub fn in29(&self) -> IN29_R {
        IN29_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN25"]
    #[inline(always)]
    pub fn in25(&mut self) -> IN25_W {
        IN25_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN26"]
    #[inline(always)]
    pub fn in26(&mut self) -> IN26_W {
        IN26_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN27"]
    #[inline(always)]
    pub fn in27(&mut self) -> IN27_W {
        IN27_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN28"]
    #[inline(always)]
    pub fn in28(&mut self) -> IN28_W {
        IN28_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN29"]
    #[inline(always)]
    pub fn in29(&mut self) -> IN29_W {
        IN29_W { w: self }
    }
}
