#[doc = "Reader of register ALERT_EN"]
pub type R = crate::R<u32, super::ALERT_EN>;
#[doc = "Writer for register ALERT_EN"]
pub type W = crate::W<u32, super::ALERT_EN>;
#[doc = "Register ALERT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ALERT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN_A0`"]
pub type EN_A0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_A0`"]
pub struct EN_A0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_A0_W<'a> {
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
#[doc = "Reader of field `EN_A1`"]
pub type EN_A1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_A1`"]
pub struct EN_A1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_A1_W<'a> {
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
#[doc = "Reader of field `EN_A2`"]
pub type EN_A2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_A2`"]
pub struct EN_A2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_A2_W<'a> {
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
#[doc = "Reader of field `EN_A3`"]
pub type EN_A3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_A3`"]
pub struct EN_A3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_A3_W<'a> {
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
    #[doc = "Bit 0 - Alert enable for alert0"]
    #[inline(always)]
    pub fn en_a0(&self) -> EN_A0_R {
        EN_A0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for alert1"]
    #[inline(always)]
    pub fn en_a1(&self) -> EN_A1_R {
        EN_A1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for alert2"]
    #[inline(always)]
    pub fn en_a2(&self) -> EN_A2_R {
        EN_A2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for alert3"]
    #[inline(always)]
    pub fn en_a3(&self) -> EN_A3_R {
        EN_A3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alert enable for alert0"]
    #[inline(always)]
    pub fn en_a0(&mut self) -> EN_A0_W {
        EN_A0_W { w: self }
    }
    #[doc = "Bit 1 - for alert1"]
    #[inline(always)]
    pub fn en_a1(&mut self) -> EN_A1_W {
        EN_A1_W { w: self }
    }
    #[doc = "Bit 2 - for alert2"]
    #[inline(always)]
    pub fn en_a2(&mut self) -> EN_A2_W {
        EN_A2_W { w: self }
    }
    #[doc = "Bit 3 - for alert3"]
    #[inline(always)]
    pub fn en_a3(&mut self) -> EN_A3_W {
        EN_A3_W { w: self }
    }
}
