#[doc = "Writer for register KEY5"]
pub type W = crate::W<u32, super::KEY5>;
#[doc = "Register KEY5 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `key5`"]
pub struct KEY5_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Key for KEY5"]
    #[inline(always)]
    pub fn key5(&mut self) -> KEY5_W {
        KEY5_W { w: self }
    }
}
