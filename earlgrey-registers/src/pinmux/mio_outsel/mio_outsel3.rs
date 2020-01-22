#[doc = "Reader of register MIO_OUTSEL3"]
pub type R = crate::R<u32, super::MIO_OUTSEL3>;
#[doc = "Writer for register MIO_OUTSEL3"]
pub type W = crate::W<u32, super::MIO_OUTSEL3>;
#[doc = "Register MIO_OUTSEL3 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT15`"]
pub type OUT15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT15`"]
pub struct OUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT16`"]
pub type OUT16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT16`"]
pub struct OUT16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT17`"]
pub type OUT17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT17`"]
pub struct OUT17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT18`"]
pub type OUT18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT18`"]
pub struct OUT18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT19`"]
pub type OUT19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT19`"]
pub struct OUT19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT15"]
    #[inline(always)]
    pub fn out15(&self) -> OUT15_R {
        OUT15_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT16"]
    #[inline(always)]
    pub fn out16(&self) -> OUT16_R {
        OUT16_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT17"]
    #[inline(always)]
    pub fn out17(&self) -> OUT17_R {
        OUT17_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT18"]
    #[inline(always)]
    pub fn out18(&self) -> OUT18_R {
        OUT18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT19"]
    #[inline(always)]
    pub fn out19(&self) -> OUT19_R {
        OUT19_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT15"]
    #[inline(always)]
    pub fn out15(&mut self) -> OUT15_W {
        OUT15_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT16"]
    #[inline(always)]
    pub fn out16(&mut self) -> OUT16_W {
        OUT16_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT17"]
    #[inline(always)]
    pub fn out17(&mut self) -> OUT17_W {
        OUT17_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT18"]
    #[inline(always)]
    pub fn out18(&mut self) -> OUT18_W {
        OUT18_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT19"]
    #[inline(always)]
    pub fn out19(&mut self) -> OUT19_W {
        OUT19_W { w: self }
    }
}
