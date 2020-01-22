#[doc = "Reader of register REGEN"]
pub type R = crate::R<u32, super::REGEN>;
#[doc = "Writer for register REGEN"]
pub type W = crate::W<u32, super::REGEN>;
#[doc = "Register REGEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::REGEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `wen`"]
pub type WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wen`"]
pub struct WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN_W<'a> {
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
    #[doc = "Bit 0 - When true, all configuration registers can be modified. When false, they become read-only. Defaults true, write zero to clear."]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When true, all configuration registers can be modified. When false, they become read-only. Defaults true, write zero to clear."]
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W {
        WEN_W { w: self }
    }
}
