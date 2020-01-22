#[doc = "Reader of register INTR_STATE"]
pub type R = crate::R<u32, super::INTR_STATE>;
#[doc = "Writer for register INTR_STATE"]
pub type W = crate::W<u32, super::INTR_STATE>;
#[doc = "Register INTR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_STATE {
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
    #[doc = "Bits 0:31 - raised if any of GPIO pin detects configured interrupt mode"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - raised if any of GPIO pin detects configured interrupt mode"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
}
