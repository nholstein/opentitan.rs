#[doc = "Reader of register MIO_OUTSEL5"]
pub type R = crate::R<u32, super::MIO_OUTSEL5>;
#[doc = "Writer for register MIO_OUTSEL5"]
pub type W = crate::W<u32, super::MIO_OUTSEL5>;
#[doc = "Register MIO_OUTSEL5 `reset()`'s with value 0x0208_2082"]
impl crate::ResetValue for super::MIO_OUTSEL5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2082
    }
}
#[doc = "Reader of field `OUT25`"]
pub type OUT25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT25`"]
pub struct OUT25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT26`"]
pub type OUT26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT26`"]
pub struct OUT26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `OUT27`"]
pub type OUT27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT27`"]
pub struct OUT27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUT28`"]
pub type OUT28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT28`"]
pub struct OUT28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUT29`"]
pub type OUT29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT29`"]
pub struct OUT29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT25"]
    #[inline(always)]
    pub fn out25(&self) -> OUT25_R {
        OUT25_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT26"]
    #[inline(always)]
    pub fn out26(&self) -> OUT26_R {
        OUT26_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for OUT27"]
    #[inline(always)]
    pub fn out27(&self) -> OUT27_R {
        OUT27_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for OUT28"]
    #[inline(always)]
    pub fn out28(&self) -> OUT28_R {
        OUT28_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for OUT29"]
    #[inline(always)]
    pub fn out29(&self) -> OUT29_R {
        OUT29_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT25"]
    #[inline(always)]
    pub fn out25(&mut self) -> OUT25_W {
        OUT25_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT26"]
    #[inline(always)]
    pub fn out26(&mut self) -> OUT26_W {
        OUT26_W { w: self }
    }
    #[doc = "Bits 12:17 - for OUT27"]
    #[inline(always)]
    pub fn out27(&mut self) -> OUT27_W {
        OUT27_W { w: self }
    }
    #[doc = "Bits 18:23 - for OUT28"]
    #[inline(always)]
    pub fn out28(&mut self) -> OUT28_W {
        OUT28_W { w: self }
    }
    #[doc = "Bits 24:29 - for OUT29"]
    #[inline(always)]
    pub fn out29(&mut self) -> OUT29_W {
        OUT29_W { w: self }
    }
}
