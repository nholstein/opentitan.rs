#[doc = "Reader of register MIO_OUTSEL2"]
pub type R = crate::R<u32, super::MIO_OUTSEL2>;
#[doc = "Writer for register MIO_OUTSEL2"]
pub type W = crate::W<u32, super::MIO_OUTSEL2>;
#[doc = "Register MIO_OUTSEL2 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT10`"]
pub type OUT10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT10`"]
pub struct OUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT11`"]
pub type OUT11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT11`"]
pub struct OUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT12`"]
pub type OUT12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT12`"]
pub struct OUT12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT13`"]
pub type OUT13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT13`"]
pub struct OUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT14`"]
pub type OUT14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT14`"]
pub struct OUT14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT10"]
    #[inline(always)]
    pub fn out10(&self) -> OUT10_R {
        OUT10_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT11"]
    #[inline(always)]
    pub fn out11(&self) -> OUT11_R {
        OUT11_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT12"]
    #[inline(always)]
    pub fn out12(&self) -> OUT12_R {
        OUT12_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT13"]
    #[inline(always)]
    pub fn out13(&self) -> OUT13_R {
        OUT13_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT14"]
    #[inline(always)]
    pub fn out14(&self) -> OUT14_R {
        OUT14_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT10"]
    #[inline(always)]
    pub fn out10(&mut self) -> OUT10_W {
        OUT10_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT11"]
    #[inline(always)]
    pub fn out11(&mut self) -> OUT11_W {
        OUT11_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT12"]
    #[inline(always)]
    pub fn out12(&mut self) -> OUT12_W {
        OUT12_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT13"]
    #[inline(always)]
    pub fn out13(&mut self) -> OUT13_W {
        OUT13_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT14"]
    #[inline(always)]
    pub fn out14(&mut self) -> OUT14_W {
        OUT14_W { w: self }
    }
}
