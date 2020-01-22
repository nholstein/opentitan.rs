#[doc = "Reader of register INTR_ENABLE0"]
pub type R = crate::R<u32, super::INTR_ENABLE0>;
#[doc = "Writer for register INTR_ENABLE0"]
pub type W = crate::W<u32, super::INTR_ENABLE0>;
#[doc = "Register INTR_ENABLE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_ENABLE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IE0`"]
pub type IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE0`"]
pub struct IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IE0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable for timer for TIMER0"]
    #[inline(always)]
    pub fn ie0(&self) -> IE0_R {
        IE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable for timer for TIMER0"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W {
        IE0_W { w: self }
    }
}
