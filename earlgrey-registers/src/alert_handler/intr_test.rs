#[doc = "Writer for register INTR_TEST"]
pub type W = crate::W<u32, super::INTR_TEST>;
#[doc = "Register INTR_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Write 1 to force !!INTR_STATE.classa to 1"]
    #[inline(always)]
    pub fn classa(&mut self) -> CLASSA_W {
        CLASSA_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to force !!INTR_STATE.classb to 1"]
    #[inline(always)]
    pub fn classb(&mut self) -> CLASSB_W {
        CLASSB_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 to force !!INTR_STATE.classc to 1"]
    #[inline(always)]
    pub fn classc(&mut self) -> CLASSC_W {
        CLASSC_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 to force !!INTR_STATE.classd to 1"]
    #[inline(always)]
    pub fn classd(&mut self) -> CLASSD_W {
        CLASSD_W { w: self }
    }
}
