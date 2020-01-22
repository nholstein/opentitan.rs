#[doc = "Writer for register KEY7"]
pub type W = crate::W<u32, super::KEY7>;
#[doc = "Register KEY7 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `key7`"]
pub struct KEY7_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Secret Key for HMAC7"]
    #[inline(always)]
    pub fn key7(&mut self) -> KEY7_W {
        KEY7_W { w: self }
    }
}
