#[doc = "Writer for register WIPE_SECRET"]
pub type W = crate::W<u32, super::WIPE_SECRET>;
#[doc = "Register WIPE_SECRET `reset()`'s with value 0"]
impl crate::ResetValue for super::WIPE_SECRET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `secret`"]
pub struct SECRET_W<'a> {
    w: &'a mut W,
}
impl<'a> SECRET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Secret value"]
    #[inline(always)]
    pub fn secret(&mut self) -> SECRET_W {
        SECRET_W { w: self }
    }
}
