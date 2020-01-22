#[doc = "Reader of register LOC_ALERT_CAUSE"]
pub type R = crate::R<u32, super::LOC_ALERT_CAUSE>;
#[doc = "Writer for register LOC_ALERT_CAUSE"]
pub type W = crate::W<u32, super::LOC_ALERT_CAUSE>;
#[doc = "Register LOC_ALERT_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::LOC_ALERT_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LA0`"]
pub type LA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LA0`"]
pub struct LA0_W<'a> {
    w: &'a mut W,
}
impl<'a> LA0_W<'a> {
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
#[doc = "Reader of field `LA1`"]
pub type LA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LA1`"]
pub struct LA1_W<'a> {
    w: &'a mut W,
}
impl<'a> LA1_W<'a> {
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
#[doc = "Reader of field `LA2`"]
pub type LA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LA2`"]
pub struct LA2_W<'a> {
    w: &'a mut W,
}
impl<'a> LA2_W<'a> {
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
#[doc = "Reader of field `LA3`"]
pub type LA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LA3`"]
pub struct LA3_W<'a> {
    w: &'a mut W,
}
impl<'a> LA3_W<'a> {
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
    #[doc = "Bit 0 - Cause bit for LOC_ALERT0"]
    #[inline(always)]
    pub fn la0(&self) -> LA0_R {
        LA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for LOC_ALERT1"]
    #[inline(always)]
    pub fn la1(&self) -> LA1_R {
        LA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for LOC_ALERT2"]
    #[inline(always)]
    pub fn la2(&self) -> LA2_R {
        LA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for LOC_ALERT3"]
    #[inline(always)]
    pub fn la3(&self) -> LA3_R {
        LA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cause bit for LOC_ALERT0"]
    #[inline(always)]
    pub fn la0(&mut self) -> LA0_W {
        LA0_W { w: self }
    }
    #[doc = "Bit 1 - for LOC_ALERT1"]
    #[inline(always)]
    pub fn la1(&mut self) -> LA1_W {
        LA1_W { w: self }
    }
    #[doc = "Bit 2 - for LOC_ALERT2"]
    #[inline(always)]
    pub fn la2(&mut self) -> LA2_W {
        LA2_W { w: self }
    }
    #[doc = "Bit 3 - for LOC_ALERT3"]
    #[inline(always)]
    pub fn la3(&mut self) -> LA3_W {
        LA3_W { w: self }
    }
}
