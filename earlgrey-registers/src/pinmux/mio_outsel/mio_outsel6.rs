#[doc = "Reader of register MIO_OUTSEL6"]
pub type R = crate::R<u32, super::MIO_OUTSEL6>;
#[doc = "Writer for register MIO_OUTSEL6"]
pub type W = crate::W<u32, super::MIO_OUTSEL6>;
#[doc = "Register MIO_OUTSEL6 `reset()`'s with value 0x82"]
impl crate::ResetValue for super::MIO_OUTSEL6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x82
    }
}
#[doc = "Reader of field `OUT30`"]
pub type OUT30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT30`"]
pub struct OUT30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `OUT31`"]
pub type OUT31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT31`"]
pub struct OUT31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT30"]
    #[inline(always)]
    pub fn out30(&self) -> OUT30_R {
        OUT30_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for OUT31"]
    #[inline(always)]
    pub fn out31(&self) -> OUT31_R {
        OUT31_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. 2: high-Z >=3: peripheral outputs (i.e., add 3 to the native peripheral pad index). for OUT30"]
    #[inline(always)]
    pub fn out30(&mut self) -> OUT30_W {
        OUT30_W { w: self }
    }
    #[doc = "Bits 6:11 - for OUT31"]
    #[inline(always)]
    pub fn out31(&mut self) -> OUT31_W {
        OUT31_W { w: self }
    }
}
