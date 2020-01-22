#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Reader of field `KEY_LEN`"]
pub type KEY_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_LEN`"]
pub struct KEY_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `MANUAL_START_TRIGGER`"]
pub type MANUAL_START_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MANUAL_START_TRIGGER`"]
pub struct MANUAL_START_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUAL_START_TRIGGER_W<'a> {
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
#[doc = "Reader of field `FORCE_DATA_OVERWRITE`"]
pub type FORCE_DATA_OVERWRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_DATA_OVERWRITE`"]
pub struct FORCE_DATA_OVERWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DATA_OVERWRITE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Select encryption(0) or decryption(1) operating mode of AES unit."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - 3-bit one-hot field to select AES key length: 128 bit (3'b001), 192 bit (3'b010) or 256 bit (3'b100). Invalid input values, i.e., value with multiple bits set, value 3'b000, and value 3'b010 in case 192-bit keys are not supported (because disabled at compile time) are mapped to 3'b001."]
    #[inline(always)]
    pub fn key_len(&self) -> KEY_LEN_R {
        KEY_LEN_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Controls whether the AES unit should automatically start to encrypt/decrypt when it receives new input data (0) or wait for separate trigger signal before starting (1) (see Trigger Register)."]
    #[inline(always)]
    pub fn manual_start_trigger(&self) -> MANUAL_START_TRIGGER_R {
        MANUAL_START_TRIGGER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control whether the AES unit is stalled during the last encryption/decryption cycle if the previous output data has not yet been read (0) or finishes the operation and overwrites the previous output data (1)."]
    #[inline(always)]
    pub fn force_data_overwrite(&self) -> FORCE_DATA_OVERWRITE_R {
        FORCE_DATA_OVERWRITE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select encryption(0) or decryption(1) operating mode of AES unit."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:3 - 3-bit one-hot field to select AES key length: 128 bit (3'b001), 192 bit (3'b010) or 256 bit (3'b100). Invalid input values, i.e., value with multiple bits set, value 3'b000, and value 3'b010 in case 192-bit keys are not supported (because disabled at compile time) are mapped to 3'b001."]
    #[inline(always)]
    pub fn key_len(&mut self) -> KEY_LEN_W {
        KEY_LEN_W { w: self }
    }
    #[doc = "Bit 4 - Controls whether the AES unit should automatically start to encrypt/decrypt when it receives new input data (0) or wait for separate trigger signal before starting (1) (see Trigger Register)."]
    #[inline(always)]
    pub fn manual_start_trigger(&mut self) -> MANUAL_START_TRIGGER_W {
        MANUAL_START_TRIGGER_W { w: self }
    }
    #[doc = "Bit 5 - Control whether the AES unit is stalled during the last encryption/decryption cycle if the previous output data has not yet been read (0) or finishes the operation and overwrites the previous output data (1)."]
    #[inline(always)]
    pub fn force_data_overwrite(&mut self) -> FORCE_DATA_OVERWRITE_W {
        FORCE_DATA_OVERWRITE_W { w: self }
    }
}
