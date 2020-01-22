#[doc = "Reader of register PERIPH_INSEL6"]
pub type R = crate::R<u32, super::PERIPH_INSEL6>;
#[doc = "Writer for register PERIPH_INSEL6"]
pub type W = crate::W<u32, super::PERIPH_INSEL6>;
#[doc = "Register PERIPH_INSEL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN30`"]
pub type IN30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN30`"]
pub struct IN30_W<'a> {
    w: &'a mut W,
}
impl<'a> IN30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN31`"]
pub type IN31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN31`"]
pub struct IN31_W<'a> {
    w: &'a mut W,
}
impl<'a> IN31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN30"]
    #[inline(always)]
    pub fn in30(&self) -> IN30_R {
        IN30_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN31"]
    #[inline(always)]
    pub fn in31(&self) -> IN31_R {
        IN31_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN30"]
    #[inline(always)]
    pub fn in30(&mut self) -> IN30_W {
        IN30_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN31"]
    #[inline(always)]
    pub fn in31(&mut self) -> IN31_W {
        IN31_W { w: self }
    }
}
