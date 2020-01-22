#[doc = "Writer for register DATA_IN3"]
pub type W = crate::W<u32, super::DATA_IN3>;
#[doc = "Register DATA_IN3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_IN3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `data_in3`"]
pub struct DATA_IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for DATA_IN3"]
    #[inline(always)]
    pub fn data_in3(&mut self) -> DATA_IN3_W {
        DATA_IN3_W { w: self }
    }
}
