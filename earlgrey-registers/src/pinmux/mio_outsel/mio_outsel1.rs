#[doc = "Reader of register MIO_OUTSEL1"]
pub type R = crate::R<u32, super::MIO_OUTSEL1>;
#[doc = "Writer for register MIO_OUTSEL1"]
pub type W = crate::W<u32, super::MIO_OUTSEL1>;
#[doc = "Register MIO_OUTSEL1 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT5`"]
pub type OUT5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT5`"]
pub struct OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT6`"]
pub type OUT6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT6`"]
pub struct OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT7`"]
pub type OUT7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT7`"]
pub struct OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT8`"]
pub type OUT8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT8`"]
pub struct OUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT9`"]
pub type OUT9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT9`"]
pub struct OUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT5"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT6"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT7"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT8"]
    #[inline(always)]
    pub fn out8(&self) -> OUT8_R {
        OUT8_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT9"]
    #[inline(always)]
    pub fn out9(&self) -> OUT9_R {
        OUT9_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT5"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W {
        OUT5_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT6"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W {
        OUT6_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT7"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W {
        OUT7_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT8"]
    #[inline(always)]
    pub fn out8(&mut self) -> OUT8_W {
        OUT8_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT9"]
    #[inline(always)]
    pub fn out9(&mut self) -> OUT9_W {
        OUT9_W { w: self }
    }
}
