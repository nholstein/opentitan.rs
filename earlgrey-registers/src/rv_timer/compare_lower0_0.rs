#[doc = "Reader of register COMPARE_LOWER0_0"]
pub type R = crate::R<u32, super::COMPARE_LOWER0_0>;
#[doc = "Writer for register COMPARE_LOWER0_0"]
pub type W = crate::W<u32, super::COMPARE_LOWER0_0>;
#[doc = "Register COMPARE_LOWER0_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::COMPARE_LOWER0_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `v`"]
pub type V_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `v`"]
pub struct V_W<'a> {
    w: &'a mut W,
}
impl<'a> V_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer compare value \\[31:0\\]"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer compare value \\[31:0\\]"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W {
        V_W { w: self }
    }
}
