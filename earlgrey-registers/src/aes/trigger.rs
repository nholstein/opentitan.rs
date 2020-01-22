#[doc = "Writer for register TRIGGER"]
pub type W = crate::W<u32, super::TRIGGER>;
#[doc = "Register TRIGGER `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIGGER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Write proxy for field `KEY_CLEAR`"]
pub struct KEY_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_CLEAR_W<'a> {
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
#[doc = "Write proxy for field `DATA_IN_CLEAR`"]
pub struct DATA_IN_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_CLEAR_W<'a> {
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
#[doc = "Write proxy for field `DATA_OUT_CLEAR`"]
pub struct DATA_OUT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_OUT_CLEAR_W<'a> {
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
    #[doc = "Bit 0 - Keep AES unit paused (0) or trigger the encryption/decryption of one data block (1)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Keep current values in Initial Key, internal Full Key and Decryption Key registers (0) or clear those registers (1)."]
    #[inline(always)]
    pub fn key_clear(&mut self) -> KEY_CLEAR_W {
        KEY_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Keep current values in input registers (0) or clear those registers (1)."]
    #[inline(always)]
    pub fn data_in_clear(&mut self) -> DATA_IN_CLEAR_W {
        DATA_IN_CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Keep current values in output registers (0) or clear those registers (1)."]
    #[inline(always)]
    pub fn data_out_clear(&mut self) -> DATA_OUT_CLEAR_W {
        DATA_OUT_CLEAR_W { w: self }
    }
}
