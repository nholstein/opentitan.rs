#[doc = "Reader of register LE0"]
pub type R = crate::R<u32, super::LE0>;
#[doc = "Writer for register LE0"]
pub type W = crate::W<u32, super::LE0>;
#[doc = "Register LE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LE0`"]
pub type LE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE0`"]
pub struct LE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LE0_W<'a> {
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
#[doc = "Reader of field `LE1`"]
pub type LE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE1`"]
pub struct LE1_W<'a> {
    w: &'a mut W,
}
impl<'a> LE1_W<'a> {
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
#[doc = "Reader of field `LE2`"]
pub type LE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE2`"]
pub struct LE2_W<'a> {
    w: &'a mut W,
}
impl<'a> LE2_W<'a> {
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
#[doc = "Reader of field `LE3`"]
pub type LE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE3`"]
pub struct LE3_W<'a> {
    w: &'a mut W,
}
impl<'a> LE3_W<'a> {
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
#[doc = "Reader of field `LE4`"]
pub type LE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE4`"]
pub struct LE4_W<'a> {
    w: &'a mut W,
}
impl<'a> LE4_W<'a> {
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
#[doc = "Reader of field `LE5`"]
pub type LE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE5`"]
pub struct LE5_W<'a> {
    w: &'a mut W,
}
impl<'a> LE5_W<'a> {
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
#[doc = "Reader of field `LE6`"]
pub type LE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE6`"]
pub struct LE6_W<'a> {
    w: &'a mut W,
}
impl<'a> LE6_W<'a> {
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
#[doc = "Reader of field `LE7`"]
pub type LE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE7`"]
pub struct LE7_W<'a> {
    w: &'a mut W,
}
impl<'a> LE7_W<'a> {
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
#[doc = "Reader of field `LE8`"]
pub type LE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE8`"]
pub struct LE8_W<'a> {
    w: &'a mut W,
}
impl<'a> LE8_W<'a> {
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
#[doc = "Reader of field `LE9`"]
pub type LE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE9`"]
pub struct LE9_W<'a> {
    w: &'a mut W,
}
impl<'a> LE9_W<'a> {
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
#[doc = "Reader of field `LE10`"]
pub type LE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE10`"]
pub struct LE10_W<'a> {
    w: &'a mut W,
}
impl<'a> LE10_W<'a> {
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
#[doc = "Reader of field `LE11`"]
pub type LE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE11`"]
pub struct LE11_W<'a> {
    w: &'a mut W,
}
impl<'a> LE11_W<'a> {
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
#[doc = "Reader of field `LE12`"]
pub type LE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE12`"]
pub struct LE12_W<'a> {
    w: &'a mut W,
}
impl<'a> LE12_W<'a> {
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
#[doc = "Reader of field `LE13`"]
pub type LE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE13`"]
pub struct LE13_W<'a> {
    w: &'a mut W,
}
impl<'a> LE13_W<'a> {
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
#[doc = "Reader of field `LE14`"]
pub type LE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE14`"]
pub struct LE14_W<'a> {
    w: &'a mut W,
}
impl<'a> LE14_W<'a> {
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
#[doc = "Reader of field `LE15`"]
pub type LE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE15`"]
pub struct LE15_W<'a> {
    w: &'a mut W,
}
impl<'a> LE15_W<'a> {
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
#[doc = "Reader of field `LE16`"]
pub type LE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE16`"]
pub struct LE16_W<'a> {
    w: &'a mut W,
}
impl<'a> LE16_W<'a> {
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
#[doc = "Reader of field `LE17`"]
pub type LE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE17`"]
pub struct LE17_W<'a> {
    w: &'a mut W,
}
impl<'a> LE17_W<'a> {
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
#[doc = "Reader of field `LE18`"]
pub type LE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE18`"]
pub struct LE18_W<'a> {
    w: &'a mut W,
}
impl<'a> LE18_W<'a> {
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
#[doc = "Reader of field `LE19`"]
pub type LE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE19`"]
pub struct LE19_W<'a> {
    w: &'a mut W,
}
impl<'a> LE19_W<'a> {
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
#[doc = "Reader of field `LE20`"]
pub type LE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE20`"]
pub struct LE20_W<'a> {
    w: &'a mut W,
}
impl<'a> LE20_W<'a> {
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
#[doc = "Reader of field `LE21`"]
pub type LE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE21`"]
pub struct LE21_W<'a> {
    w: &'a mut W,
}
impl<'a> LE21_W<'a> {
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
#[doc = "Reader of field `LE22`"]
pub type LE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE22`"]
pub struct LE22_W<'a> {
    w: &'a mut W,
}
impl<'a> LE22_W<'a> {
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
#[doc = "Reader of field `LE23`"]
pub type LE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE23`"]
pub struct LE23_W<'a> {
    w: &'a mut W,
}
impl<'a> LE23_W<'a> {
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
#[doc = "Reader of field `LE24`"]
pub type LE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE24`"]
pub struct LE24_W<'a> {
    w: &'a mut W,
}
impl<'a> LE24_W<'a> {
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
#[doc = "Reader of field `LE25`"]
pub type LE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE25`"]
pub struct LE25_W<'a> {
    w: &'a mut W,
}
impl<'a> LE25_W<'a> {
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
#[doc = "Reader of field `LE26`"]
pub type LE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE26`"]
pub struct LE26_W<'a> {
    w: &'a mut W,
}
impl<'a> LE26_W<'a> {
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
#[doc = "Reader of field `LE27`"]
pub type LE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE27`"]
pub struct LE27_W<'a> {
    w: &'a mut W,
}
impl<'a> LE27_W<'a> {
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
#[doc = "Reader of field `LE28`"]
pub type LE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE28`"]
pub struct LE28_W<'a> {
    w: &'a mut W,
}
impl<'a> LE28_W<'a> {
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
#[doc = "Reader of field `LE29`"]
pub type LE29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE29`"]
pub struct LE29_W<'a> {
    w: &'a mut W,
}
impl<'a> LE29_W<'a> {
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
#[doc = "Reader of field `LE30`"]
pub type LE30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE30`"]
pub struct LE30_W<'a> {
    w: &'a mut W,
}
impl<'a> LE30_W<'a> {
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
#[doc = "Reader of field `LE31`"]
pub type LE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE31`"]
pub struct LE31_W<'a> {
    w: &'a mut W,
}
impl<'a> LE31_W<'a> {
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
    #[doc = "Bit 0 - L0E1 for RV_PLIC0"]
    #[inline(always)]
    pub fn le0(&self) -> LE0_R {
        LE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC1"]
    #[inline(always)]
    pub fn le1(&self) -> LE1_R {
        LE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC2"]
    #[inline(always)]
    pub fn le2(&self) -> LE2_R {
        LE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC3"]
    #[inline(always)]
    pub fn le3(&self) -> LE3_R {
        LE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC4"]
    #[inline(always)]
    pub fn le4(&self) -> LE4_R {
        LE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC5"]
    #[inline(always)]
    pub fn le5(&self) -> LE5_R {
        LE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC6"]
    #[inline(always)]
    pub fn le6(&self) -> LE6_R {
        LE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC7"]
    #[inline(always)]
    pub fn le7(&self) -> LE7_R {
        LE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC8"]
    #[inline(always)]
    pub fn le8(&self) -> LE8_R {
        LE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC9"]
    #[inline(always)]
    pub fn le9(&self) -> LE9_R {
        LE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC10"]
    #[inline(always)]
    pub fn le10(&self) -> LE10_R {
        LE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC11"]
    #[inline(always)]
    pub fn le11(&self) -> LE11_R {
        LE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC12"]
    #[inline(always)]
    pub fn le12(&self) -> LE12_R {
        LE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC13"]
    #[inline(always)]
    pub fn le13(&self) -> LE13_R {
        LE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC14"]
    #[inline(always)]
    pub fn le14(&self) -> LE14_R {
        LE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC15"]
    #[inline(always)]
    pub fn le15(&self) -> LE15_R {
        LE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC16"]
    #[inline(always)]
    pub fn le16(&self) -> LE16_R {
        LE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC17"]
    #[inline(always)]
    pub fn le17(&self) -> LE17_R {
        LE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC18"]
    #[inline(always)]
    pub fn le18(&self) -> LE18_R {
        LE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC19"]
    #[inline(always)]
    pub fn le19(&self) -> LE19_R {
        LE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC20"]
    #[inline(always)]
    pub fn le20(&self) -> LE20_R {
        LE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC21"]
    #[inline(always)]
    pub fn le21(&self) -> LE21_R {
        LE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC22"]
    #[inline(always)]
    pub fn le22(&self) -> LE22_R {
        LE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC23"]
    #[inline(always)]
    pub fn le23(&self) -> LE23_R {
        LE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC24"]
    #[inline(always)]
    pub fn le24(&self) -> LE24_R {
        LE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC25"]
    #[inline(always)]
    pub fn le25(&self) -> LE25_R {
        LE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC26"]
    #[inline(always)]
    pub fn le26(&self) -> LE26_R {
        LE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC27"]
    #[inline(always)]
    pub fn le27(&self) -> LE27_R {
        LE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC28"]
    #[inline(always)]
    pub fn le28(&self) -> LE28_R {
        LE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC29"]
    #[inline(always)]
    pub fn le29(&self) -> LE29_R {
        LE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC30"]
    #[inline(always)]
    pub fn le30(&self) -> LE30_R {
        LE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - for RV_PLIC31"]
    #[inline(always)]
    pub fn le31(&self) -> LE31_R {
        LE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L0E1 for RV_PLIC0"]
    #[inline(always)]
    pub fn le0(&mut self) -> LE0_W {
        LE0_W { w: self }
    }
    #[doc = "Bit 1 - for RV_PLIC1"]
    #[inline(always)]
    pub fn le1(&mut self) -> LE1_W {
        LE1_W { w: self }
    }
    #[doc = "Bit 2 - for RV_PLIC2"]
    #[inline(always)]
    pub fn le2(&mut self) -> LE2_W {
        LE2_W { w: self }
    }
    #[doc = "Bit 3 - for RV_PLIC3"]
    #[inline(always)]
    pub fn le3(&mut self) -> LE3_W {
        LE3_W { w: self }
    }
    #[doc = "Bit 4 - for RV_PLIC4"]
    #[inline(always)]
    pub fn le4(&mut self) -> LE4_W {
        LE4_W { w: self }
    }
    #[doc = "Bit 5 - for RV_PLIC5"]
    #[inline(always)]
    pub fn le5(&mut self) -> LE5_W {
        LE5_W { w: self }
    }
    #[doc = "Bit 6 - for RV_PLIC6"]
    #[inline(always)]
    pub fn le6(&mut self) -> LE6_W {
        LE6_W { w: self }
    }
    #[doc = "Bit 7 - for RV_PLIC7"]
    #[inline(always)]
    pub fn le7(&mut self) -> LE7_W {
        LE7_W { w: self }
    }
    #[doc = "Bit 8 - for RV_PLIC8"]
    #[inline(always)]
    pub fn le8(&mut self) -> LE8_W {
        LE8_W { w: self }
    }
    #[doc = "Bit 9 - for RV_PLIC9"]
    #[inline(always)]
    pub fn le9(&mut self) -> LE9_W {
        LE9_W { w: self }
    }
    #[doc = "Bit 10 - for RV_PLIC10"]
    #[inline(always)]
    pub fn le10(&mut self) -> LE10_W {
        LE10_W { w: self }
    }
    #[doc = "Bit 11 - for RV_PLIC11"]
    #[inline(always)]
    pub fn le11(&mut self) -> LE11_W {
        LE11_W { w: self }
    }
    #[doc = "Bit 12 - for RV_PLIC12"]
    #[inline(always)]
    pub fn le12(&mut self) -> LE12_W {
        LE12_W { w: self }
    }
    #[doc = "Bit 13 - for RV_PLIC13"]
    #[inline(always)]
    pub fn le13(&mut self) -> LE13_W {
        LE13_W { w: self }
    }
    #[doc = "Bit 14 - for RV_PLIC14"]
    #[inline(always)]
    pub fn le14(&mut self) -> LE14_W {
        LE14_W { w: self }
    }
    #[doc = "Bit 15 - for RV_PLIC15"]
    #[inline(always)]
    pub fn le15(&mut self) -> LE15_W {
        LE15_W { w: self }
    }
    #[doc = "Bit 16 - for RV_PLIC16"]
    #[inline(always)]
    pub fn le16(&mut self) -> LE16_W {
        LE16_W { w: self }
    }
    #[doc = "Bit 17 - for RV_PLIC17"]
    #[inline(always)]
    pub fn le17(&mut self) -> LE17_W {
        LE17_W { w: self }
    }
    #[doc = "Bit 18 - for RV_PLIC18"]
    #[inline(always)]
    pub fn le18(&mut self) -> LE18_W {
        LE18_W { w: self }
    }
    #[doc = "Bit 19 - for RV_PLIC19"]
    #[inline(always)]
    pub fn le19(&mut self) -> LE19_W {
        LE19_W { w: self }
    }
    #[doc = "Bit 20 - for RV_PLIC20"]
    #[inline(always)]
    pub fn le20(&mut self) -> LE20_W {
        LE20_W { w: self }
    }
    #[doc = "Bit 21 - for RV_PLIC21"]
    #[inline(always)]
    pub fn le21(&mut self) -> LE21_W {
        LE21_W { w: self }
    }
    #[doc = "Bit 22 - for RV_PLIC22"]
    #[inline(always)]
    pub fn le22(&mut self) -> LE22_W {
        LE22_W { w: self }
    }
    #[doc = "Bit 23 - for RV_PLIC23"]
    #[inline(always)]
    pub fn le23(&mut self) -> LE23_W {
        LE23_W { w: self }
    }
    #[doc = "Bit 24 - for RV_PLIC24"]
    #[inline(always)]
    pub fn le24(&mut self) -> LE24_W {
        LE24_W { w: self }
    }
    #[doc = "Bit 25 - for RV_PLIC25"]
    #[inline(always)]
    pub fn le25(&mut self) -> LE25_W {
        LE25_W { w: self }
    }
    #[doc = "Bit 26 - for RV_PLIC26"]
    #[inline(always)]
    pub fn le26(&mut self) -> LE26_W {
        LE26_W { w: self }
    }
    #[doc = "Bit 27 - for RV_PLIC27"]
    #[inline(always)]
    pub fn le27(&mut self) -> LE27_W {
        LE27_W { w: self }
    }
    #[doc = "Bit 28 - for RV_PLIC28"]
    #[inline(always)]
    pub fn le28(&mut self) -> LE28_W {
        LE28_W { w: self }
    }
    #[doc = "Bit 29 - for RV_PLIC29"]
    #[inline(always)]
    pub fn le29(&mut self) -> LE29_W {
        LE29_W { w: self }
    }
    #[doc = "Bit 30 - for RV_PLIC30"]
    #[inline(always)]
    pub fn le30(&mut self) -> LE30_W {
        LE30_W { w: self }
    }
    #[doc = "Bit 31 - for RV_PLIC31"]
    #[inline(always)]
    pub fn le31(&mut self) -> LE31_W {
        LE31_W { w: self }
    }
}
