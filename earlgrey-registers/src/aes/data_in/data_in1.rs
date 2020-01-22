#[doc = "Writer for register DATA_IN1"]
pub type W = crate::W<u32, super::DATA_IN1>;
#[doc = "Register DATA_IN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_IN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `data_in1`"]
pub struct DATA_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for DATA_IN1"]
    #[inline(always)]
    pub fn data_in1(&mut self) -> DATA_IN1_W {
        DATA_IN1_W { w: self }
    }
}
