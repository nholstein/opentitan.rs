#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `active0`"]
pub type ACTIVE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `active0`"]
pub struct ACTIVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE0_W<'a> {
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
    #[doc = "Bit 0 - If 1, timer operates for TIMER0"]
    #[inline(always)]
    pub fn active0(&self) -> ACTIVE0_R {
        ACTIVE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, timer operates for TIMER0"]
    #[inline(always)]
    pub fn active0(&mut self) -> ACTIVE0_W {
        ACTIVE0_W { w: self }
    }
}
