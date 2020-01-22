#[doc = "Reader of register INTR_STATE"]
pub type R = crate::R<u32, super::INTR_STATE>;
#[doc = "Writer for register INTR_STATE"]
pub type W = crate::W<u32, super::INTR_STATE>;
#[doc = "Register INTR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `classa`"]
pub type CLASSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `classa`"]
pub struct CLASSA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSA_W<'a> {
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
#[doc = "Reader of field `classb`"]
pub type CLASSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `classb`"]
pub struct CLASSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSB_W<'a> {
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
#[doc = "Reader of field `classc`"]
pub type CLASSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `classc`"]
pub struct CLASSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSC_W<'a> {
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
#[doc = "Reader of field `classd`"]
pub type CLASSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `classd`"]
pub struct CLASSD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASSD_W<'a> {
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
    #[doc = "Bit 0 - Interrupt state bit of Class A. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classa(&self) -> CLASSA_R {
        CLASSA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt state bit of Class B. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classb(&self) -> CLASSB_R {
        CLASSB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt state bit of Class C. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classc(&self) -> CLASSC_R {
        CLASSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt state bit of Class D. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classd(&self) -> CLASSD_R {
        CLASSD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt state bit of Class A. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classa(&mut self) -> CLASSA_W {
        CLASSA_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt state bit of Class B. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classb(&mut self) -> CLASSB_W {
        CLASSB_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt state bit of Class C. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classc(&mut self) -> CLASSC_W {
        CLASSC_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt state bit of Class D. Set by HW in case an alert within this class triggered. Defaults true, write one to clear."]
    #[inline(always)]
    pub fn classd(&mut self) -> CLASSD_W {
        CLASSD_W { w: self }
    }
}
