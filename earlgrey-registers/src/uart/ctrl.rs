#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
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
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
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
#[doc = "Reader of field `NF`"]
pub type NF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NF`"]
pub struct NF_W<'a> {
    w: &'a mut W,
}
impl<'a> NF_W<'a> {
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
#[doc = "Reader of field `SLPBK`"]
pub type SLPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLPBK`"]
pub struct SLPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPBK_W<'a> {
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
#[doc = "Reader of field `LLPBK`"]
pub type LLPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLPBK`"]
pub struct LLPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LLPBK_W<'a> {
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
#[doc = "Reader of field `PARITY_EN`"]
pub type PARITY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_EN`"]
pub struct PARITY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PARITY_ODD`"]
pub type PARITY_ODD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_ODD`"]
pub struct PARITY_ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ODD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RXBLVL`"]
pub type RXBLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBLVL`"]
pub struct RXBLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `NCO`"]
pub type NCO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NCO`"]
pub struct NCO_W<'a> {
    w: &'a mut W,
}
impl<'a> NCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX enable"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX enable"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX noise filter enable. If the noise filter is enabled, RX line goes through the 3-tap repetition code. It ignores single IP clock period noise."]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System loopback enable. If this bit is turned on, any outgoing bits to TX are received through RX. See Block Diagram. Note that the TX line goes 1 if System loopback is enabled."]
    #[inline(always)]
    pub fn slpbk(&self) -> SLPBK_R {
        SLPBK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line loopback enable. If this bit is turned on, incoming bits are forwarded to TX for testing purpose. See Block Diagram. Note that the internal design sees RX value as 1 always if line loopback is enabled."]
    #[inline(always)]
    pub fn llpbk(&self) -> LLPBK_R {
        LLPBK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If true, parity is enabled in both RX and TX directions."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If PARITY_EN is true, this determines the type, 1 for odd parity, 0 for even."]
    #[inline(always)]
    pub fn parity_odd(&self) -> PARITY_ODD_R {
        PARITY_ODD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Trigger level for RX break detection. Sets the number of character times the line must be low to detect a break."]
    #[inline(always)]
    pub fn rxblvl(&self) -> RXBLVL_R {
        RXBLVL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - BAUD clock rate control."]
    #[inline(always)]
    pub fn nco(&self) -> NCO_R {
        NCO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX enable"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 1 - RX enable"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
    #[doc = "Bit 2 - RX noise filter enable. If the noise filter is enabled, RX line goes through the 3-tap repetition code. It ignores single IP clock period noise."]
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W {
        NF_W { w: self }
    }
    #[doc = "Bit 4 - System loopback enable. If this bit is turned on, any outgoing bits to TX are received through RX. See Block Diagram. Note that the TX line goes 1 if System loopback is enabled."]
    #[inline(always)]
    pub fn slpbk(&mut self) -> SLPBK_W {
        SLPBK_W { w: self }
    }
    #[doc = "Bit 5 - Line loopback enable. If this bit is turned on, incoming bits are forwarded to TX for testing purpose. See Block Diagram. Note that the internal design sees RX value as 1 always if line loopback is enabled."]
    #[inline(always)]
    pub fn llpbk(&mut self) -> LLPBK_W {
        LLPBK_W { w: self }
    }
    #[doc = "Bit 6 - If true, parity is enabled in both RX and TX directions."]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W {
        PARITY_EN_W { w: self }
    }
    #[doc = "Bit 7 - If PARITY_EN is true, this determines the type, 1 for odd parity, 0 for even."]
    #[inline(always)]
    pub fn parity_odd(&mut self) -> PARITY_ODD_W {
        PARITY_ODD_W { w: self }
    }
    #[doc = "Bits 8:9 - Trigger level for RX break detection. Sets the number of character times the line must be low to detect a break."]
    #[inline(always)]
    pub fn rxblvl(&mut self) -> RXBLVL_W {
        RXBLVL_W { w: self }
    }
    #[doc = "Bits 16:31 - BAUD clock rate control."]
    #[inline(always)]
    pub fn nco(&mut self) -> NCO_W {
        NCO_W { w: self }
    }
}
