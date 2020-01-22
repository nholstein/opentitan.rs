#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x04"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `hmac_en`"]
pub type HMAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hmac_en`"]
pub struct HMAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HMAC_EN_W<'a> {
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
#[doc = "Reader of field `sha_en`"]
pub type SHA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sha_en`"]
pub struct SHA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_EN_W<'a> {
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
#[doc = "Reader of field `endian_swap`"]
pub type ENDIAN_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `endian_swap`"]
pub struct ENDIAN_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_SWAP_W<'a> {
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
#[doc = "Reader of field `digest_swap`"]
pub type DIGEST_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `digest_swap`"]
pub struct DIGEST_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGEST_SWAP_W<'a> {
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
    #[doc = "Bit 0 - HMAC datapath enable. If this bit is 1, HMAC operates when `hash_start` toggles."]
    #[inline(always)]
    pub fn hmac_en(&self) -> HMAC_EN_R {
        HMAC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SHA256 enable. If 0, SHA engine won't initiate compression, this is used to stop operation of the SHA engine until configuration has been done. When the SHA engine is disabled the digest is cleared."]
    #[inline(always)]
    pub fn sha_en(&self) -> SHA_EN_R {
        SHA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Convert TL-UL wdata\\[31:0\\]
to big-endian style {w\\[7:0\\], w\\[15:8\\], .. }"]
    #[inline(always)]
    pub fn endian_swap(&self) -> ENDIAN_SWAP_R {
        ENDIAN_SWAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIGEST register byte-order. If 1, it swaps each DIGEST registers' byte-order."]
    #[inline(always)]
    pub fn digest_swap(&self) -> DIGEST_SWAP_R {
        DIGEST_SWAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HMAC datapath enable. If this bit is 1, HMAC operates when `hash_start` toggles."]
    #[inline(always)]
    pub fn hmac_en(&mut self) -> HMAC_EN_W {
        HMAC_EN_W { w: self }
    }
    #[doc = "Bit 1 - SHA256 enable. If 0, SHA engine won't initiate compression, this is used to stop operation of the SHA engine until configuration has been done. When the SHA engine is disabled the digest is cleared."]
    #[inline(always)]
    pub fn sha_en(&mut self) -> SHA_EN_W {
        SHA_EN_W { w: self }
    }
    #[doc = "Bit 2 - Convert TL-UL wdata\\[31:0\\]
to big-endian style {w\\[7:0\\], w\\[15:8\\], .. }"]
    #[inline(always)]
    pub fn endian_swap(&mut self) -> ENDIAN_SWAP_W {
        ENDIAN_SWAP_W { w: self }
    }
    #[doc = "Bit 3 - DIGEST register byte-order. If 1, it swaps each DIGEST registers' byte-order."]
    #[inline(always)]
    pub fn digest_swap(&mut self) -> DIGEST_SWAP_W {
        DIGEST_SWAP_W { w: self }
    }
}
