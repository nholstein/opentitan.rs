#[doc = "Reader of register OVRD"]
pub type R = crate::R<u32, super::OVRD>;
#[doc = "Writer for register OVRD"]
pub type W = crate::W<u32, super::OVRD>;
#[doc = "Register OVRD `reset()`'s with value 0"]
impl crate::ResetValue for super::OVRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Reader of field `TXVAL`"]
pub type TXVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXVAL`"]
pub struct TXVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable TX pin override control"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write to set the value of the TX pin"]
    #[inline(always)]
    pub fn txval(&self) -> TXVAL_R {
        TXVAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TX pin override control"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 1 - Write to set the value of the TX pin"]
    #[inline(always)]
    pub fn txval(&mut self) -> TXVAL_W {
        TXVAL_W { w: self }
    }
}
