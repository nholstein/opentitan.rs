#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `hash_start`"]
pub struct HASH_START_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_START_W<'a> {
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
#[doc = "Write proxy for field `hash_process`"]
pub struct HASH_PROCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_PROCESS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - If writes 1 into this field, SHA256 or HMAC begins its operation. CPU should configure relative information first, such as message_length, secret_key."]
    #[inline(always)]
    pub fn hash_start(&mut self) -> HASH_START_W {
        HASH_START_W { w: self }
    }
    #[doc = "Bit 1 - If writes 1 into this field, SHA256 or HMAC calculates the digest or signing based on currently received message."]
    #[inline(always)]
    pub fn hash_process(&mut self) -> HASH_PROCESS_W {
        HASH_PROCESS_W { w: self }
    }
}
