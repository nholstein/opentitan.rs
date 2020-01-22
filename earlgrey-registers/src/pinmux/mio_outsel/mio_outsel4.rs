#[doc = "Reader of register MIO_OUTSEL4"]
pub type R = crate::R<u32, super::MIO_OUTSEL4>;
#[doc = "Writer for register MIO_OUTSEL4"]
pub type W = crate::W<u32, super::MIO_OUTSEL4>;
#[doc = "Register MIO_OUTSEL4 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT20`"]
pub type OUT20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT20`"]
pub struct OUT20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT21`"]
pub type OUT21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT21`"]
pub struct OUT21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT22`"]
pub type OUT22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT22`"]
pub struct OUT22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT23`"]
pub type OUT23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT23`"]
pub struct OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT24`"]
pub type OUT24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT24`"]
pub struct OUT24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT20"]
    #[inline(always)]
    pub fn out20(&self) -> OUT20_R {
        OUT20_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT21"]
    #[inline(always)]
    pub fn out21(&self) -> OUT21_R {
        OUT21_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT22"]
    #[inline(always)]
    pub fn out22(&self) -> OUT22_R {
        OUT22_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT23"]
    #[inline(always)]
    pub fn out23(&self) -> OUT23_R {
        OUT23_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT24"]
    #[inline(always)]
    pub fn out24(&self) -> OUT24_R {
        OUT24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT20"]
    #[inline(always)]
    pub fn out20(&mut self) -> OUT20_W {
        OUT20_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT21"]
    #[inline(always)]
    pub fn out21(&mut self) -> OUT21_W {
        OUT21_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT22"]
    #[inline(always)]
    pub fn out22(&mut self) -> OUT22_W {
        OUT22_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT23"]
    #[inline(always)]
    pub fn out23(&mut self) -> OUT23_W {
        OUT23_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT24"]
    #[inline(always)]
    pub fn out24(&mut self) -> OUT24_W {
        OUT24_W { w: self }
    }
}
