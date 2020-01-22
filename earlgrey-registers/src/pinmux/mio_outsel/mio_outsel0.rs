#[doc = "Reader of register MIO_OUTSEL0"]
pub type R = crate::R<u32, super::MIO_OUTSEL0>;
#[doc = "Writer for register MIO_OUTSEL0"]
pub type W = crate::W<u32, super::MIO_OUTSEL0>;
#[doc = "Register MIO_OUTSEL0 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT0`"]
pub type OUT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT0`"]
pub struct OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT1`"]
pub type OUT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT1`"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT2`"]
pub type OUT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT2`"]
pub struct OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT3`"]
pub type OUT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT3`"]
pub struct OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT4`"]
pub type OUT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT4`"]
pub struct OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT0"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT3"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT4"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT0"]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W {
        OUT0_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT3"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W {
        OUT3_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT4"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W {
        OUT4_W { w: self }
    }
}
