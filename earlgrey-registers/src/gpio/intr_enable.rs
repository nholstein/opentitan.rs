#[doc = "Reader of register INTR_ENABLE"]
pub type R = crate::R<u32, super::INTR_ENABLE>;
#[doc = "Writer for register INTR_ENABLE"]
pub type W = crate::W<u32, super::INTR_ENABLE>;
#[doc = "Register INTR_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpio`"]
pub type GPIO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpio`"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Enable interrupt when corresponding bit in !!INTR_STATE.gpio is set"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable interrupt when corresponding bit in !!INTR_STATE.gpio is set"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
}
