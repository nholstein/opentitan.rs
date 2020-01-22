#[doc = "Reader of register BANK_CFG_REGWEN"]
pub type R = crate::R<u32, super::BANK_CFG_REGWEN>;
#[doc = "Writer for register BANK_CFG_REGWEN"]
pub type W = crate::W<u32, super::BANK_CFG_REGWEN>;
#[doc = "Register BANK_CFG_REGWEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BANK_CFG_REGWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `BANK`"]
pub type BANK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK`"]
pub struct BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_W<'a> {
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
    #[doc = "Bit 0 - Bank register write enable. Once set to 0, it can longer be configured to 1"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank register write enable. Once set to 0, it can longer be configured to 1"]
    #[inline(always)]
    pub fn bank(&mut self) -> BANK_W {
        BANK_W { w: self }
    }
}
