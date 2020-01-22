#[doc = "Reader of register IE00"]
pub type R = crate::R<u32, super::IE00>;
#[doc = "Writer for register IE00"]
pub type W = crate::W<u32, super::IE00>;
#[doc = "Register IE00 `reset()`'s with value 0"]
impl crate::ResetValue for super::IE00 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `E0`"]
pub type E0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E0`"]
pub struct E0_W<'a> {
    w: &'a mut W,
}
impl<'a> E0_W<'a> {
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
#[doc = "Reader of field `E1`"]
pub type E1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E1`"]
pub struct E1_W<'a> {
    w: &'a mut W,
}
impl<'a> E1_W<'a> {
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
#[doc = "Reader of field `E2`"]
pub type E2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E2`"]
pub struct E2_W<'a> {
    w: &'a mut W,
}
impl<'a> E2_W<'a> {
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
#[doc = "Reader of field `E3`"]
pub type E3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E3`"]
pub struct E3_W<'a> {
    w: &'a mut W,
}
impl<'a> E3_W<'a> {
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
#[doc = "Reader of field `E4`"]
pub type E4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E4`"]
pub struct E4_W<'a> {
    w: &'a mut W,
}
impl<'a> E4_W<'a> {
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
#[doc = "Reader of field `E5`"]
pub type E5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E5`"]
pub struct E5_W<'a> {
    w: &'a mut W,
}
impl<'a> E5_W<'a> {
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
#[doc = "Reader of field `E6`"]
pub type E6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E6`"]
pub struct E6_W<'a> {
    w: &'a mut W,
}
impl<'a> E6_W<'a> {
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
#[doc = "Reader of field `E7`"]
pub type E7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E7`"]
pub struct E7_W<'a> {
    w: &'a mut W,
}
impl<'a> E7_W<'a> {
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
#[doc = "Reader of field `E8`"]
pub type E8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E8`"]
pub struct E8_W<'a> {
    w: &'a mut W,
}
impl<'a> E8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `E9`"]
pub type E9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E9`"]
pub struct E9_W<'a> {
    w: &'a mut W,
}
impl<'a> E9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `E10`"]
pub type E10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E10`"]
pub struct E10_W<'a> {
    w: &'a mut W,
}
impl<'a> E10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `E11`"]
pub type E11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E11`"]
pub struct E11_W<'a> {
    w: &'a mut W,
}
impl<'a> E11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `E12`"]
pub type E12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E12`"]
pub struct E12_W<'a> {
    w: &'a mut W,
}
impl<'a> E12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `E13`"]
pub type E13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E13`"]
pub struct E13_W<'a> {
    w: &'a mut W,
}
impl<'a> E13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `E14`"]
pub type E14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E14`"]
pub struct E14_W<'a> {
    w: &'a mut W,
}
impl<'a> E14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `E15`"]
pub type E15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E15`"]
pub struct E15_W<'a> {
    w: &'a mut W,
}
impl<'a> E15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `E16`"]
pub type E16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E16`"]
pub struct E16_W<'a> {
    w: &'a mut W,
}
impl<'a> E16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `E17`"]
pub type E17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E17`"]
pub struct E17_W<'a> {
    w: &'a mut W,
}
impl<'a> E17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `E18`"]
pub type E18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E18`"]
pub struct E18_W<'a> {
    w: &'a mut W,
}
impl<'a> E18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `E19`"]
pub type E19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E19`"]
pub struct E19_W<'a> {
    w: &'a mut W,
}
impl<'a> E19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `E20`"]
pub type E20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E20`"]
pub struct E20_W<'a> {
    w: &'a mut W,
}
impl<'a> E20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `E21`"]
pub type E21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E21`"]
pub struct E21_W<'a> {
    w: &'a mut W,
}
impl<'a> E21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `E22`"]
pub type E22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E22`"]
pub struct E22_W<'a> {
    w: &'a mut W,
}
impl<'a> E22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `E23`"]
pub type E23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E23`"]
pub struct E23_W<'a> {
    w: &'a mut W,
}
impl<'a> E23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `E24`"]
pub type E24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E24`"]
pub struct E24_W<'a> {
    w: &'a mut W,
}
impl<'a> E24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `E25`"]
pub type E25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E25`"]
pub struct E25_W<'a> {
    w: &'a mut W,
}
impl<'a> E25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `E26`"]
pub type E26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E26`"]
pub struct E26_W<'a> {
    w: &'a mut W,
}
impl<'a> E26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `E27`"]
pub type E27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E27`"]
pub struct E27_W<'a> {
    w: &'a mut W,
}
impl<'a> E27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `E28`"]
pub type E28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E28`"]
pub struct E28_W<'a> {
    w: &'a mut W,
}
impl<'a> E28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `E29`"]
pub type E29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E29`"]
pub struct E29_W<'a> {
    w: &'a mut W,
}
impl<'a> E29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `E30`"]
pub type E30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E30`"]
pub struct E30_W<'a> {
    w: &'a mut W,
}
impl<'a> E30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `E31`"]
pub type E31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E31`"]
pub struct E31_W<'a> {
    w: &'a mut W,
}
impl<'a> E31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable of Source for RV_PLIC0"]
    #[inline(always)]
    pub fn e0(&self) -> E0_R {
        E0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC1"]
    #[inline(always)]
    pub fn e1(&self) -> E1_R {
        E1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC2"]
    #[inline(always)]
    pub fn e2(&self) -> E2_R {
        E2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC3"]
    #[inline(always)]
    pub fn e3(&self) -> E3_R {
        E3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC4"]
    #[inline(always)]
    pub fn e4(&self) -> E4_R {
        E4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC5"]
    #[inline(always)]
    pub fn e5(&self) -> E5_R {
        E5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC6"]
    #[inline(always)]
    pub fn e6(&self) -> E6_R {
        E6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC7"]
    #[inline(always)]
    pub fn e7(&self) -> E7_R {
        E7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC8"]
    #[inline(always)]
    pub fn e8(&self) -> E8_R {
        E8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC9"]
    #[inline(always)]
    pub fn e9(&self) -> E9_R {
        E9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC10"]
    #[inline(always)]
    pub fn e10(&self) -> E10_R {
        E10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC11"]
    #[inline(always)]
    pub fn e11(&self) -> E11_R {
        E11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC12"]
    #[inline(always)]
    pub fn e12(&self) -> E12_R {
        E12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC13"]
    #[inline(always)]
    pub fn e13(&self) -> E13_R {
        E13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC14"]
    #[inline(always)]
    pub fn e14(&self) -> E14_R {
        E14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC15"]
    #[inline(always)]
    pub fn e15(&self) -> E15_R {
        E15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC16"]
    #[inline(always)]
    pub fn e16(&self) -> E16_R {
        E16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC17"]
    #[inline(always)]
    pub fn e17(&self) -> E17_R {
        E17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC18"]
    #[inline(always)]
    pub fn e18(&self) -> E18_R {
        E18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC19"]
    #[inline(always)]
    pub fn e19(&self) -> E19_R {
        E19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC20"]
    #[inline(always)]
    pub fn e20(&self) -> E20_R {
        E20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC21"]
    #[inline(always)]
    pub fn e21(&self) -> E21_R {
        E21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC22"]
    #[inline(always)]
    pub fn e22(&self) -> E22_R {
        E22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC23"]
    #[inline(always)]
    pub fn e23(&self) -> E23_R {
        E23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC24"]
    #[inline(always)]
    pub fn e24(&self) -> E24_R {
        E24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC25"]
    #[inline(always)]
    pub fn e25(&self) -> E25_R {
        E25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC26"]
    #[inline(always)]
    pub fn e26(&self) -> E26_R {
        E26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC27"]
    #[inline(always)]
    pub fn e27(&self) -> E27_R {
        E27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC28"]
    #[inline(always)]
    pub fn e28(&self) -> E28_R {
        E28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC29"]
    #[inline(always)]
    pub fn e29(&self) -> E29_R {
        E29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC30"]
    #[inline(always)]
    pub fn e30(&self) -> E30_R {
        E30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - for RV_PLIC31"]
    #[inline(always)]
    pub fn e31(&self) -> E31_R {
        E31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable of Source for RV_PLIC0"]
    #[inline(always)]
    pub fn e0(&mut self) -> E0_W {
        E0_W { w: self }
    }
    #[doc = "Bit 1 - for RV_PLIC1"]
    #[inline(always)]
    pub fn e1(&mut self) -> E1_W {
        E1_W { w: self }
    }
    #[doc = "Bit 2 - for RV_PLIC2"]
    #[inline(always)]
    pub fn e2(&mut self) -> E2_W {
        E2_W { w: self }
    }
    #[doc = "Bit 3 - for RV_PLIC3"]
    #[inline(always)]
    pub fn e3(&mut self) -> E3_W {
        E3_W { w: self }
    }
    #[doc = "Bit 4 - for RV_PLIC4"]
    #[inline(always)]
    pub fn e4(&mut self) -> E4_W {
        E4_W { w: self }
    }
    #[doc = "Bit 5 - for RV_PLIC5"]
    #[inline(always)]
    pub fn e5(&mut self) -> E5_W {
        E5_W { w: self }
    }
    #[doc = "Bit 6 - for RV_PLIC6"]
    #[inline(always)]
    pub fn e6(&mut self) -> E6_W {
        E6_W { w: self }
    }
    #[doc = "Bit 7 - for RV_PLIC7"]
    #[inline(always)]
    pub fn e7(&mut self) -> E7_W {
        E7_W { w: self }
    }
    #[doc = "Bit 8 - for RV_PLIC8"]
    #[inline(always)]
    pub fn e8(&mut self) -> E8_W {
        E8_W { w: self }
    }
    #[doc = "Bit 9 - for RV_PLIC9"]
    #[inline(always)]
    pub fn e9(&mut self) -> E9_W {
        E9_W { w: self }
    }
    #[doc = "Bit 10 - for RV_PLIC10"]
    #[inline(always)]
    pub fn e10(&mut self) -> E10_W {
        E10_W { w: self }
    }
    #[doc = "Bit 11 - for RV_PLIC11"]
    #[inline(always)]
    pub fn e11(&mut self) -> E11_W {
        E11_W { w: self }
    }
    #[doc = "Bit 12 - for RV_PLIC12"]
    #[inline(always)]
    pub fn e12(&mut self) -> E12_W {
        E12_W { w: self }
    }
    #[doc = "Bit 13 - for RV_PLIC13"]
    #[inline(always)]
    pub fn e13(&mut self) -> E13_W {
        E13_W { w: self }
    }
    #[doc = "Bit 14 - for RV_PLIC14"]
    #[inline(always)]
    pub fn e14(&mut self) -> E14_W {
        E14_W { w: self }
    }
    #[doc = "Bit 15 - for RV_PLIC15"]
    #[inline(always)]
    pub fn e15(&mut self) -> E15_W {
        E15_W { w: self }
    }
    #[doc = "Bit 16 - for RV_PLIC16"]
    #[inline(always)]
    pub fn e16(&mut self) -> E16_W {
        E16_W { w: self }
    }
    #[doc = "Bit 17 - for RV_PLIC17"]
    #[inline(always)]
    pub fn e17(&mut self) -> E17_W {
        E17_W { w: self }
    }
    #[doc = "Bit 18 - for RV_PLIC18"]
    #[inline(always)]
    pub fn e18(&mut self) -> E18_W {
        E18_W { w: self }
    }
    #[doc = "Bit 19 - for RV_PLIC19"]
    #[inline(always)]
    pub fn e19(&mut self) -> E19_W {
        E19_W { w: self }
    }
    #[doc = "Bit 20 - for RV_PLIC20"]
    #[inline(always)]
    pub fn e20(&mut self) -> E20_W {
        E20_W { w: self }
    }
    #[doc = "Bit 21 - for RV_PLIC21"]
    #[inline(always)]
    pub fn e21(&mut self) -> E21_W {
        E21_W { w: self }
    }
    #[doc = "Bit 22 - for RV_PLIC22"]
    #[inline(always)]
    pub fn e22(&mut self) -> E22_W {
        E22_W { w: self }
    }
    #[doc = "Bit 23 - for RV_PLIC23"]
    #[inline(always)]
    pub fn e23(&mut self) -> E23_W {
        E23_W { w: self }
    }
    #[doc = "Bit 24 - for RV_PLIC24"]
    #[inline(always)]
    pub fn e24(&mut self) -> E24_W {
        E24_W { w: self }
    }
    #[doc = "Bit 25 - for RV_PLIC25"]
    #[inline(always)]
    pub fn e25(&mut self) -> E25_W {
        E25_W { w: self }
    }
    #[doc = "Bit 26 - for RV_PLIC26"]
    #[inline(always)]
    pub fn e26(&mut self) -> E26_W {
        E26_W { w: self }
    }
    #[doc = "Bit 27 - for RV_PLIC27"]
    #[inline(always)]
    pub fn e27(&mut self) -> E27_W {
        E27_W { w: self }
    }
    #[doc = "Bit 28 - for RV_PLIC28"]
    #[inline(always)]
    pub fn e28(&mut self) -> E28_W {
        E28_W { w: self }
    }
    #[doc = "Bit 29 - for RV_PLIC29"]
    #[inline(always)]
    pub fn e29(&mut self) -> E29_W {
        E29_W { w: self }
    }
    #[doc = "Bit 30 - for RV_PLIC30"]
    #[inline(always)]
    pub fn e30(&mut self) -> E30_W {
        E30_W { w: self }
    }
    #[doc = "Bit 31 - for RV_PLIC31"]
    #[inline(always)]
    pub fn e31(&mut self) -> E31_W {
        E31_W { w: self }
    }
}
