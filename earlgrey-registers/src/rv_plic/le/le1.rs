#[doc = "Reader of register LE1"]
pub type R = crate::R<u32, super::LE1>;
#[doc = "Writer for register LE1"]
pub type W = crate::W<u32, super::LE1>;
#[doc = "Register LE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LE32`"]
pub type LE32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE32`"]
pub struct LE32_W<'a> {
    w: &'a mut W,
}
impl<'a> LE32_W<'a> {
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
#[doc = "Reader of field `LE33`"]
pub type LE33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE33`"]
pub struct LE33_W<'a> {
    w: &'a mut W,
}
impl<'a> LE33_W<'a> {
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
#[doc = "Reader of field `LE34`"]
pub type LE34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE34`"]
pub struct LE34_W<'a> {
    w: &'a mut W,
}
impl<'a> LE34_W<'a> {
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
#[doc = "Reader of field `LE35`"]
pub type LE35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE35`"]
pub struct LE35_W<'a> {
    w: &'a mut W,
}
impl<'a> LE35_W<'a> {
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
#[doc = "Reader of field `LE36`"]
pub type LE36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE36`"]
pub struct LE36_W<'a> {
    w: &'a mut W,
}
impl<'a> LE36_W<'a> {
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
#[doc = "Reader of field `LE37`"]
pub type LE37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE37`"]
pub struct LE37_W<'a> {
    w: &'a mut W,
}
impl<'a> LE37_W<'a> {
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
#[doc = "Reader of field `LE38`"]
pub type LE38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE38`"]
pub struct LE38_W<'a> {
    w: &'a mut W,
}
impl<'a> LE38_W<'a> {
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
#[doc = "Reader of field `LE39`"]
pub type LE39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE39`"]
pub struct LE39_W<'a> {
    w: &'a mut W,
}
impl<'a> LE39_W<'a> {
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
#[doc = "Reader of field `LE40`"]
pub type LE40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE40`"]
pub struct LE40_W<'a> {
    w: &'a mut W,
}
impl<'a> LE40_W<'a> {
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
#[doc = "Reader of field `LE41`"]
pub type LE41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE41`"]
pub struct LE41_W<'a> {
    w: &'a mut W,
}
impl<'a> LE41_W<'a> {
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
#[doc = "Reader of field `LE42`"]
pub type LE42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE42`"]
pub struct LE42_W<'a> {
    w: &'a mut W,
}
impl<'a> LE42_W<'a> {
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
#[doc = "Reader of field `LE43`"]
pub type LE43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE43`"]
pub struct LE43_W<'a> {
    w: &'a mut W,
}
impl<'a> LE43_W<'a> {
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
#[doc = "Reader of field `LE44`"]
pub type LE44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE44`"]
pub struct LE44_W<'a> {
    w: &'a mut W,
}
impl<'a> LE44_W<'a> {
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
#[doc = "Reader of field `LE45`"]
pub type LE45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE45`"]
pub struct LE45_W<'a> {
    w: &'a mut W,
}
impl<'a> LE45_W<'a> {
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
#[doc = "Reader of field `LE46`"]
pub type LE46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE46`"]
pub struct LE46_W<'a> {
    w: &'a mut W,
}
impl<'a> LE46_W<'a> {
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
#[doc = "Reader of field `LE47`"]
pub type LE47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE47`"]
pub struct LE47_W<'a> {
    w: &'a mut W,
}
impl<'a> LE47_W<'a> {
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
#[doc = "Reader of field `LE48`"]
pub type LE48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE48`"]
pub struct LE48_W<'a> {
    w: &'a mut W,
}
impl<'a> LE48_W<'a> {
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
#[doc = "Reader of field `LE49`"]
pub type LE49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE49`"]
pub struct LE49_W<'a> {
    w: &'a mut W,
}
impl<'a> LE49_W<'a> {
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
#[doc = "Reader of field `LE50`"]
pub type LE50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE50`"]
pub struct LE50_W<'a> {
    w: &'a mut W,
}
impl<'a> LE50_W<'a> {
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
#[doc = "Reader of field `LE51`"]
pub type LE51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE51`"]
pub struct LE51_W<'a> {
    w: &'a mut W,
}
impl<'a> LE51_W<'a> {
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
#[doc = "Reader of field `LE52`"]
pub type LE52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE52`"]
pub struct LE52_W<'a> {
    w: &'a mut W,
}
impl<'a> LE52_W<'a> {
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
#[doc = "Reader of field `LE53`"]
pub type LE53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE53`"]
pub struct LE53_W<'a> {
    w: &'a mut W,
}
impl<'a> LE53_W<'a> {
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
#[doc = "Reader of field `LE54`"]
pub type LE54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE54`"]
pub struct LE54_W<'a> {
    w: &'a mut W,
}
impl<'a> LE54_W<'a> {
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
#[doc = "Reader of field `LE55`"]
pub type LE55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE55`"]
pub struct LE55_W<'a> {
    w: &'a mut W,
}
impl<'a> LE55_W<'a> {
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
#[doc = "Reader of field `LE56`"]
pub type LE56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE56`"]
pub struct LE56_W<'a> {
    w: &'a mut W,
}
impl<'a> LE56_W<'a> {
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
#[doc = "Reader of field `LE57`"]
pub type LE57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE57`"]
pub struct LE57_W<'a> {
    w: &'a mut W,
}
impl<'a> LE57_W<'a> {
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
#[doc = "Reader of field `LE58`"]
pub type LE58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE58`"]
pub struct LE58_W<'a> {
    w: &'a mut W,
}
impl<'a> LE58_W<'a> {
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
#[doc = "Reader of field `LE59`"]
pub type LE59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE59`"]
pub struct LE59_W<'a> {
    w: &'a mut W,
}
impl<'a> LE59_W<'a> {
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
#[doc = "Reader of field `LE60`"]
pub type LE60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE60`"]
pub struct LE60_W<'a> {
    w: &'a mut W,
}
impl<'a> LE60_W<'a> {
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
#[doc = "Reader of field `LE61`"]
pub type LE61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE61`"]
pub struct LE61_W<'a> {
    w: &'a mut W,
}
impl<'a> LE61_W<'a> {
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
#[doc = "Reader of field `LE62`"]
pub type LE62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE62`"]
pub struct LE62_W<'a> {
    w: &'a mut W,
}
impl<'a> LE62_W<'a> {
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
impl R {
    #[doc = "Bit 0 - L0E1 for RV_PLIC32"]
    #[inline(always)]
    pub fn le32(&self) -> LE32_R {
        LE32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC33"]
    #[inline(always)]
    pub fn le33(&self) -> LE33_R {
        LE33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC34"]
    #[inline(always)]
    pub fn le34(&self) -> LE34_R {
        LE34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC35"]
    #[inline(always)]
    pub fn le35(&self) -> LE35_R {
        LE35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC36"]
    #[inline(always)]
    pub fn le36(&self) -> LE36_R {
        LE36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC37"]
    #[inline(always)]
    pub fn le37(&self) -> LE37_R {
        LE37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC38"]
    #[inline(always)]
    pub fn le38(&self) -> LE38_R {
        LE38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC39"]
    #[inline(always)]
    pub fn le39(&self) -> LE39_R {
        LE39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC40"]
    #[inline(always)]
    pub fn le40(&self) -> LE40_R {
        LE40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC41"]
    #[inline(always)]
    pub fn le41(&self) -> LE41_R {
        LE41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC42"]
    #[inline(always)]
    pub fn le42(&self) -> LE42_R {
        LE42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC43"]
    #[inline(always)]
    pub fn le43(&self) -> LE43_R {
        LE43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC44"]
    #[inline(always)]
    pub fn le44(&self) -> LE44_R {
        LE44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC45"]
    #[inline(always)]
    pub fn le45(&self) -> LE45_R {
        LE45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC46"]
    #[inline(always)]
    pub fn le46(&self) -> LE46_R {
        LE46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC47"]
    #[inline(always)]
    pub fn le47(&self) -> LE47_R {
        LE47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC48"]
    #[inline(always)]
    pub fn le48(&self) -> LE48_R {
        LE48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC49"]
    #[inline(always)]
    pub fn le49(&self) -> LE49_R {
        LE49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC50"]
    #[inline(always)]
    pub fn le50(&self) -> LE50_R {
        LE50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC51"]
    #[inline(always)]
    pub fn le51(&self) -> LE51_R {
        LE51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC52"]
    #[inline(always)]
    pub fn le52(&self) -> LE52_R {
        LE52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC53"]
    #[inline(always)]
    pub fn le53(&self) -> LE53_R {
        LE53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC54"]
    #[inline(always)]
    pub fn le54(&self) -> LE54_R {
        LE54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC55"]
    #[inline(always)]
    pub fn le55(&self) -> LE55_R {
        LE55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC56"]
    #[inline(always)]
    pub fn le56(&self) -> LE56_R {
        LE56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC57"]
    #[inline(always)]
    pub fn le57(&self) -> LE57_R {
        LE57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC58"]
    #[inline(always)]
    pub fn le58(&self) -> LE58_R {
        LE58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC59"]
    #[inline(always)]
    pub fn le59(&self) -> LE59_R {
        LE59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC60"]
    #[inline(always)]
    pub fn le60(&self) -> LE60_R {
        LE60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC61"]
    #[inline(always)]
    pub fn le61(&self) -> LE61_R {
        LE61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC62"]
    #[inline(always)]
    pub fn le62(&self) -> LE62_R {
        LE62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L0E1 for RV_PLIC32"]
    #[inline(always)]
    pub fn le32(&mut self) -> LE32_W {
        LE32_W { w: self }
    }
    #[doc = "Bit 1 - for RV_PLIC33"]
    #[inline(always)]
    pub fn le33(&mut self) -> LE33_W {
        LE33_W { w: self }
    }
    #[doc = "Bit 2 - for RV_PLIC34"]
    #[inline(always)]
    pub fn le34(&mut self) -> LE34_W {
        LE34_W { w: self }
    }
    #[doc = "Bit 3 - for RV_PLIC35"]
    #[inline(always)]
    pub fn le35(&mut self) -> LE35_W {
        LE35_W { w: self }
    }
    #[doc = "Bit 4 - for RV_PLIC36"]
    #[inline(always)]
    pub fn le36(&mut self) -> LE36_W {
        LE36_W { w: self }
    }
    #[doc = "Bit 5 - for RV_PLIC37"]
    #[inline(always)]
    pub fn le37(&mut self) -> LE37_W {
        LE37_W { w: self }
    }
    #[doc = "Bit 6 - for RV_PLIC38"]
    #[inline(always)]
    pub fn le38(&mut self) -> LE38_W {
        LE38_W { w: self }
    }
    #[doc = "Bit 7 - for RV_PLIC39"]
    #[inline(always)]
    pub fn le39(&mut self) -> LE39_W {
        LE39_W { w: self }
    }
    #[doc = "Bit 8 - for RV_PLIC40"]
    #[inline(always)]
    pub fn le40(&mut self) -> LE40_W {
        LE40_W { w: self }
    }
    #[doc = "Bit 9 - for RV_PLIC41"]
    #[inline(always)]
    pub fn le41(&mut self) -> LE41_W {
        LE41_W { w: self }
    }
    #[doc = "Bit 10 - for RV_PLIC42"]
    #[inline(always)]
    pub fn le42(&mut self) -> LE42_W {
        LE42_W { w: self }
    }
    #[doc = "Bit 11 - for RV_PLIC43"]
    #[inline(always)]
    pub fn le43(&mut self) -> LE43_W {
        LE43_W { w: self }
    }
    #[doc = "Bit 12 - for RV_PLIC44"]
    #[inline(always)]
    pub fn le44(&mut self) -> LE44_W {
        LE44_W { w: self }
    }
    #[doc = "Bit 13 - for RV_PLIC45"]
    #[inline(always)]
    pub fn le45(&mut self) -> LE45_W {
        LE45_W { w: self }
    }
    #[doc = "Bit 14 - for RV_PLIC46"]
    #[inline(always)]
    pub fn le46(&mut self) -> LE46_W {
        LE46_W { w: self }
    }
    #[doc = "Bit 15 - for RV_PLIC47"]
    #[inline(always)]
    pub fn le47(&mut self) -> LE47_W {
        LE47_W { w: self }
    }
    #[doc = "Bit 16 - for RV_PLIC48"]
    #[inline(always)]
    pub fn le48(&mut self) -> LE48_W {
        LE48_W { w: self }
    }
    #[doc = "Bit 17 - for RV_PLIC49"]
    #[inline(always)]
    pub fn le49(&mut self) -> LE49_W {
        LE49_W { w: self }
    }
    #[doc = "Bit 18 - for RV_PLIC50"]
    #[inline(always)]
    pub fn le50(&mut self) -> LE50_W {
        LE50_W { w: self }
    }
    #[doc = "Bit 19 - for RV_PLIC51"]
    #[inline(always)]
    pub fn le51(&mut self) -> LE51_W {
        LE51_W { w: self }
    }
    #[doc = "Bit 20 - for RV_PLIC52"]
    #[inline(always)]
    pub fn le52(&mut self) -> LE52_W {
        LE52_W { w: self }
    }
    #[doc = "Bit 21 - for RV_PLIC53"]
    #[inline(always)]
    pub fn le53(&mut self) -> LE53_W {
        LE53_W { w: self }
    }
    #[doc = "Bit 22 - for RV_PLIC54"]
    #[inline(always)]
    pub fn le54(&mut self) -> LE54_W {
        LE54_W { w: self }
    }
    #[doc = "Bit 23 - for RV_PLIC55"]
    #[inline(always)]
    pub fn le55(&mut self) -> LE55_W {
        LE55_W { w: self }
    }
    #[doc = "Bit 24 - for RV_PLIC56"]
    #[inline(always)]
    pub fn le56(&mut self) -> LE56_W {
        LE56_W { w: self }
    }
    #[doc = "Bit 25 - for RV_PLIC57"]
    #[inline(always)]
    pub fn le57(&mut self) -> LE57_W {
        LE57_W { w: self }
    }
    #[doc = "Bit 26 - for RV_PLIC58"]
    #[inline(always)]
    pub fn le58(&mut self) -> LE58_W {
        LE58_W { w: self }
    }
    #[doc = "Bit 27 - for RV_PLIC59"]
    #[inline(always)]
    pub fn le59(&mut self) -> LE59_W {
        LE59_W { w: self }
    }
    #[doc = "Bit 28 - for RV_PLIC60"]
    #[inline(always)]
    pub fn le60(&mut self) -> LE60_W {
        LE60_W { w: self }
    }
    #[doc = "Bit 29 - for RV_PLIC61"]
    #[inline(always)]
    pub fn le61(&mut self) -> LE61_W {
        LE61_W { w: self }
    }
    #[doc = "Bit 30 - for RV_PLIC62"]
    #[inline(always)]
    pub fn le62(&mut self) -> LE62_W {
        LE62_W { w: self }
    }
}
