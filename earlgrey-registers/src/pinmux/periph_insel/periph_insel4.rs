#[doc = "Reader of register PERIPH_INSEL4"]
pub type R = crate::R<u32, super::PERIPH_INSEL4>;
#[doc = "Writer for register PERIPH_INSEL4"]
pub type W = crate::W<u32, super::PERIPH_INSEL4>;
#[doc = "Register PERIPH_INSEL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN20`"]
pub type IN20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN20`"]
pub struct IN20_W<'a> {
    w: &'a mut W,
}
impl<'a> IN20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN21`"]
pub type IN21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN21`"]
pub struct IN21_W<'a> {
    w: &'a mut W,
}
impl<'a> IN21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN22`"]
pub type IN22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN22`"]
pub struct IN22_W<'a> {
    w: &'a mut W,
}
impl<'a> IN22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN23`"]
pub type IN23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN23`"]
pub struct IN23_W<'a> {
    w: &'a mut W,
}
impl<'a> IN23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN24`"]
pub type IN24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN24`"]
pub struct IN24_W<'a> {
    w: &'a mut W,
}
impl<'a> IN24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN20"]
    #[inline(always)]
    pub fn in20(&self) -> IN20_R {
        IN20_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN21"]
    #[inline(always)]
    pub fn in21(&self) -> IN21_R {
        IN21_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN22"]
    #[inline(always)]
    pub fn in22(&self) -> IN22_R {
        IN22_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN23"]
    #[inline(always)]
    pub fn in23(&self) -> IN23_R {
        IN23_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN24"]
    #[inline(always)]
    pub fn in24(&self) -> IN24_R {
        IN24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN20"]
    #[inline(always)]
    pub fn in20(&mut self) -> IN20_W {
        IN20_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN21"]
    #[inline(always)]
    pub fn in21(&mut self) -> IN21_W {
        IN21_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN22"]
    #[inline(always)]
    pub fn in22(&mut self) -> IN22_W {
        IN22_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN23"]
    #[inline(always)]
    pub fn in23(&mut self) -> IN23_W {
        IN23_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN24"]
    #[inline(always)]
    pub fn in24(&mut self) -> IN24_W {
        IN24_W { w: self }
    }
}
