#[doc = "Reader of register CLASSD_CTRL"]
pub type R = crate::R<u32, super::CLASSD_CTRL>;
#[doc = "Writer for register CLASSD_CTRL"]
pub type W = crate::W<u32, super::CLASSD_CTRL>;
#[doc = "Register CLASSD_CTRL `reset()`'s with value 0x393c"]
impl crate::ResetValue for super::CLASSD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x393c
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
#[doc = "Reader of field `EN_E0`"]
pub type EN_E0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_E0`"]
pub struct EN_E0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_E0_W<'a> {
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
#[doc = "Reader of field `EN_E1`"]
pub type EN_E1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_E1`"]
pub struct EN_E1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_E1_W<'a> {
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
#[doc = "Reader of field `EN_E2`"]
pub type EN_E2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_E2`"]
pub struct EN_E2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_E2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EN_E3`"]
pub type EN_E3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_E3`"]
pub struct EN_E3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_E3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MAP_E0`"]
pub type MAP_E0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAP_E0`"]
pub struct MAP_E0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_E0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MAP_E1`"]
pub type MAP_E1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAP_E1`"]
pub struct MAP_E1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_E1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MAP_E2`"]
pub type MAP_E2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAP_E2`"]
pub struct MAP_E2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_E2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MAP_E3`"]
pub type MAP_E3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAP_E3`"]
pub struct MAP_E3_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_E3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable escalation mechanisms (accumulation and interrupt timeout) for Class D. Note that interrupts can fire regardless of whether the escalation mechanisms are enabled for this class or not."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable automatic locking of escalation counter for class D. If true, there is no way to stop the escalation protocol for class D once it has been triggered."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable escalation signal 0 for Class D"]
    #[inline(always)]
    pub fn en_e0(&self) -> EN_E0_R {
        EN_E0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable escalation signal 1 for Class D"]
    #[inline(always)]
    pub fn en_e1(&self) -> EN_E1_R {
        EN_E1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable escalation signal 2 for Class D"]
    #[inline(always)]
    pub fn en_e2(&self) -> EN_E2_R {
        EN_E2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable escalation signal 3 for Class D"]
    #[inline(always)]
    pub fn en_e3(&self) -> EN_E3_R {
        EN_E3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Determines in which escalation phase escalation signal 0 shall be asserted."]
    #[inline(always)]
    pub fn map_e0(&self) -> MAP_E0_R {
        MAP_E0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Determines in which escalation phase escalation signal 1 shall be asserted."]
    #[inline(always)]
    pub fn map_e1(&self) -> MAP_E1_R {
        MAP_E1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Determine sin which escalation phase escalation signal 2 shall be asserted."]
    #[inline(always)]
    pub fn map_e2(&self) -> MAP_E2_R {
        MAP_E2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Determines in which escalation phase escalation signal 3 shall be asserted."]
    #[inline(always)]
    pub fn map_e3(&self) -> MAP_E3_R {
        MAP_E3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable escalation mechanisms (accumulation and interrupt timeout) for Class D. Note that interrupts can fire regardless of whether the escalation mechanisms are enabled for this class or not."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable automatic locking of escalation counter for class D. If true, there is no way to stop the escalation protocol for class D once it has been triggered."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 2 - Enable escalation signal 0 for Class D"]
    #[inline(always)]
    pub fn en_e0(&mut self) -> EN_E0_W {
        EN_E0_W { w: self }
    }
    #[doc = "Bit 3 - Enable escalation signal 1 for Class D"]
    #[inline(always)]
    pub fn en_e1(&mut self) -> EN_E1_W {
        EN_E1_W { w: self }
    }
    #[doc = "Bit 4 - Enable escalation signal 2 for Class D"]
    #[inline(always)]
    pub fn en_e2(&mut self) -> EN_E2_W {
        EN_E2_W { w: self }
    }
    #[doc = "Bit 5 - Enable escalation signal 3 for Class D"]
    #[inline(always)]
    pub fn en_e3(&mut self) -> EN_E3_W {
        EN_E3_W { w: self }
    }
    #[doc = "Bits 6:7 - Determines in which escalation phase escalation signal 0 shall be asserted."]
    #[inline(always)]
    pub fn map_e0(&mut self) -> MAP_E0_W {
        MAP_E0_W { w: self }
    }
    #[doc = "Bits 8:9 - Determines in which escalation phase escalation signal 1 shall be asserted."]
    #[inline(always)]
    pub fn map_e1(&mut self) -> MAP_E1_W {
        MAP_E1_W { w: self }
    }
    #[doc = "Bits 10:11 - Determine sin which escalation phase escalation signal 2 shall be asserted."]
    #[inline(always)]
    pub fn map_e2(&mut self) -> MAP_E2_W {
        MAP_E2_W { w: self }
    }
    #[doc = "Bits 12:13 - Determines in which escalation phase escalation signal 3 shall be asserted."]
    #[inline(always)]
    pub fn map_e3(&mut self) -> MAP_E3_W {
        MAP_E3_W { w: self }
    }
}
