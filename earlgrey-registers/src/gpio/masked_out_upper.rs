#[doc = "Reader of register MASKED_OUT_UPPER"]
pub type R = crate::R<u32, super::MASKED_OUT_UPPER>;
#[doc = "Writer for register MASKED_OUT_UPPER"]
pub type W = crate::W<u32, super::MASKED_OUT_UPPER>;
#[doc = "Register MASKED_OUT_UPPER `reset()`'s with value 0"]
impl crate::ResetValue for super::MASKED_OUT_UPPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `data`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Write proxy for field `mask`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write data value\\[31:16\\]. Value to write into DATA_OUT\\[i\\], valid in the presence of mask\\[i\\]==1"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write data value\\[31:16\\]. Value to write into DATA_OUT\\[i\\], valid in the presence of mask\\[i\\]==1"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:31 - Write data mask\\[31:16\\]. A value of 1 in mask\\[i\\]
allows the updating of DATA_OUT\\[i\\], 16 <= i <= 31"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
