#[doc = "Reader of register LOC_ALERT_EN"]
pub type R = crate::R<u32, super::LOC_ALERT_EN>;
#[doc = "Writer for register LOC_ALERT_EN"]
pub type W = crate::W<u32, super::LOC_ALERT_EN>;
#[doc = "Register LOC_ALERT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::LOC_ALERT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN_LA0`"]
pub type EN_LA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LA0`"]
pub struct EN_LA0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LA0_W<'a> {
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
#[doc = "Reader of field `EN_LA1`"]
pub type EN_LA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LA1`"]
pub struct EN_LA1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LA1_W<'a> {
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
#[doc = "Reader of field `EN_LA2`"]
pub type EN_LA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LA2`"]
pub struct EN_LA2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LA2_W<'a> {
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
#[doc = "Reader of field `EN_LA3`"]
pub type EN_LA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_LA3`"]
pub struct EN_LA3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LA3_W<'a> {
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
    #[doc = "Bit 0 - Alert enable for local alert0"]
    #[inline(always)]
    pub fn en_la0(&self) -> EN_LA0_R {
        EN_LA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for local alert1"]
    #[inline(always)]
    pub fn en_la1(&self) -> EN_LA1_R {
        EN_LA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for local alert2"]
    #[inline(always)]
    pub fn en_la2(&self) -> EN_LA2_R {
        EN_LA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for local alert3"]
    #[inline(always)]
    pub fn en_la3(&self) -> EN_LA3_R {
        EN_LA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alert enable for local alert0"]
    #[inline(always)]
    pub fn en_la0(&mut self) -> EN_LA0_W {
        EN_LA0_W { w: self }
    }
    #[doc = "Bit 1 - for local alert1"]
    #[inline(always)]
    pub fn en_la1(&mut self) -> EN_LA1_W {
        EN_LA1_W { w: self }
    }
    #[doc = "Bit 2 - for local alert2"]
    #[inline(always)]
    pub fn en_la2(&mut self) -> EN_LA2_W {
        EN_LA2_W { w: self }
    }
    #[doc = "Bit 3 - for local alert3"]
    #[inline(always)]
    pub fn en_la3(&mut self) -> EN_LA3_W {
        EN_LA3_W { w: self }
    }
}
