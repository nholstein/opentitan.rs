#[doc = "Writer for register KEY4"]
pub type W = crate::W<u32, super::KEY4>;
#[doc = "Register KEY4 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `key4`"]
pub struct KEY4_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Secret Key for HMAC4"]
    #[inline(always)]
    pub fn key4(&mut self) -> KEY4_W {
        KEY4_W { w: self }
    }
}
