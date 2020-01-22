#[doc = "Writer for register KEY6"]
pub type W = crate::W<u32, super::KEY6>;
#[doc = "Register KEY6 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `key6`"]
pub struct KEY6_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Secret Key for HMAC6"]
    #[inline(always)]
    pub fn key6(&mut self) -> KEY6_W {
        KEY6_W { w: self }
    }
}
