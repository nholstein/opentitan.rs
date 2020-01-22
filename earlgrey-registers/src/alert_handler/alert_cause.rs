#[doc = "Reader of register ALERT_CAUSE"]
pub type R = crate::R<u32, super::ALERT_CAUSE>;
#[doc = "Writer for register ALERT_CAUSE"]
pub type W = crate::W<u32, super::ALERT_CAUSE>;
#[doc = "Register ALERT_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::ALERT_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `A0`"]
pub type A0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A0`"]
pub struct A0_W<'a> {
    w: &'a mut W,
}
impl<'a> A0_W<'a> {
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
#[doc = "Reader of field `A1`"]
pub type A1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A1`"]
pub struct A1_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_W<'a> {
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
#[doc = "Reader of field `A2`"]
pub type A2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A2`"]
pub struct A2_W<'a> {
    w: &'a mut W,
}
impl<'a> A2_W<'a> {
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
#[doc = "Reader of field `A3`"]
pub type A3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A3`"]
pub struct A3_W<'a> {
    w: &'a mut W,
}
impl<'a> A3_W<'a> {
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
    #[doc = "Bit 0 - Cause bit for ALERT0"]
    #[inline(always)]
    pub fn a0(&self) -> A0_R {
        A0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for ALERT1"]
    #[inline(always)]
    pub fn a1(&self) -> A1_R {
        A1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for ALERT2"]
    #[inline(always)]
    pub fn a2(&self) -> A2_R {
        A2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for ALERT3"]
    #[inline(always)]
    pub fn a3(&self) -> A3_R {
        A3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cause bit for ALERT0"]
    #[inline(always)]
    pub fn a0(&mut self) -> A0_W {
        A0_W { w: self }
    }
    #[doc = "Bit 1 - for ALERT1"]
    #[inline(always)]
    pub fn a1(&mut self) -> A1_W {
        A1_W { w: self }
    }
    #[doc = "Bit 2 - for ALERT2"]
    #[inline(always)]
    pub fn a2(&mut self) -> A2_W {
        A2_W { w: self }
    }
    #[doc = "Bit 3 - for ALERT3"]
    #[inline(always)]
    pub fn a3(&mut self) -> A3_W {
        A3_W { w: self }
    }
}
