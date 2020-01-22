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
#[doc = "Reader of field `esc0`"]
pub type ESC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `esc0`"]
pub struct ESC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC0_W<'a> {
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
#[doc = "Reader of field `esc1`"]
pub type ESC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `esc1`"]
pub struct ESC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC1_W<'a> {
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
#[doc = "Reader of field `esc2`"]
pub type ESC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `esc2`"]
pub struct ESC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `esc3`"]
pub type ESC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `esc3`"]
pub struct ESC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt when !!INTR_STATE.esc0 is set"]
    #[inline(always)]
    pub fn esc0(&self) -> ESC0_R {
        ESC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt when !!INTR_STATE.esc1 is set"]
    #[inline(always)]
    pub fn esc1(&self) -> ESC1_R {
        ESC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt when !!INTR_STATE.esc2 is set"]
    #[inline(always)]
    pub fn esc2(&self) -> ESC2_R {
        ESC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt when !!INTR_STATE.esc3 is set"]
    #[inline(always)]
    pub fn esc3(&self) -> ESC3_R {
        ESC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt when !!INTR_STATE.esc0 is set"]
    #[inline(always)]
    pub fn esc0(&mut self) -> ESC0_W {
        ESC0_W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt when !!INTR_STATE.esc1 is set"]
    #[inline(always)]
    pub fn esc1(&mut self) -> ESC1_W {
        ESC1_W { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt when !!INTR_STATE.esc2 is set"]
    #[inline(always)]
    pub fn esc2(&mut self) -> ESC2_W {
        ESC2_W { w: self }
    }
    #[doc = "Bit 3 - Enable interrupt when !!INTR_STATE.esc3 is set"]
    #[inline(always)]
    pub fn esc3(&mut self) -> ESC3_W {
        ESC3_W { w: self }
    }
}
