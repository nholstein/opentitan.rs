#[doc = "Reader of register IE01"]
pub type R = crate::R<u32, super::IE01>;
#[doc = "Writer for register IE01"]
pub type W = crate::W<u32, super::IE01>;
#[doc = "Register IE01 `reset()`'s with value 0"]
impl crate::ResetValue for super::IE01 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `E32`"]
pub type E32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E32`"]
pub struct E32_W<'a> {
    w: &'a mut W,
}
impl<'a> E32_W<'a> {
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
#[doc = "Reader of field `E33`"]
pub type E33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E33`"]
pub struct E33_W<'a> {
    w: &'a mut W,
}
impl<'a> E33_W<'a> {
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
#[doc = "Reader of field `E34`"]
pub type E34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E34`"]
pub struct E34_W<'a> {
    w: &'a mut W,
}
impl<'a> E34_W<'a> {
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
#[doc = "Reader of field `E35`"]
pub type E35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E35`"]
pub struct E35_W<'a> {
    w: &'a mut W,
}
impl<'a> E35_W<'a> {
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
#[doc = "Reader of field `E36`"]
pub type E36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E36`"]
pub struct E36_W<'a> {
    w: &'a mut W,
}
impl<'a> E36_W<'a> {
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
#[doc = "Reader of field `E37`"]
pub type E37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E37`"]
pub struct E37_W<'a> {
    w: &'a mut W,
}
impl<'a> E37_W<'a> {
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
#[doc = "Reader of field `E38`"]
pub type E38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E38`"]
pub struct E38_W<'a> {
    w: &'a mut W,
}
impl<'a> E38_W<'a> {
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
#[doc = "Reader of field `E39`"]
pub type E39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E39`"]
pub struct E39_W<'a> {
    w: &'a mut W,
}
impl<'a> E39_W<'a> {
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
#[doc = "Reader of field `E40`"]
pub type E40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E40`"]
pub struct E40_W<'a> {
    w: &'a mut W,
}
impl<'a> E40_W<'a> {
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
#[doc = "Reader of field `E41`"]
pub type E41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E41`"]
pub struct E41_W<'a> {
    w: &'a mut W,
}
impl<'a> E41_W<'a> {
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
#[doc = "Reader of field `E42`"]
pub type E42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E42`"]
pub struct E42_W<'a> {
    w: &'a mut W,
}
impl<'a> E42_W<'a> {
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
#[doc = "Reader of field `E43`"]
pub type E43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E43`"]
pub struct E43_W<'a> {
    w: &'a mut W,
}
impl<'a> E43_W<'a> {
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
#[doc = "Reader of field `E44`"]
pub type E44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E44`"]
pub struct E44_W<'a> {
    w: &'a mut W,
}
impl<'a> E44_W<'a> {
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
#[doc = "Reader of field `E45`"]
pub type E45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E45`"]
pub struct E45_W<'a> {
    w: &'a mut W,
}
impl<'a> E45_W<'a> {
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
#[doc = "Reader of field `E46`"]
pub type E46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E46`"]
pub struct E46_W<'a> {
    w: &'a mut W,
}
impl<'a> E46_W<'a> {
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
#[doc = "Reader of field `E47`"]
pub type E47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E47`"]
pub struct E47_W<'a> {
    w: &'a mut W,
}
impl<'a> E47_W<'a> {
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
#[doc = "Reader of field `E48`"]
pub type E48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E48`"]
pub struct E48_W<'a> {
    w: &'a mut W,
}
impl<'a> E48_W<'a> {
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
#[doc = "Reader of field `E49`"]
pub type E49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E49`"]
pub struct E49_W<'a> {
    w: &'a mut W,
}
impl<'a> E49_W<'a> {
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
#[doc = "Reader of field `E50`"]
pub type E50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E50`"]
pub struct E50_W<'a> {
    w: &'a mut W,
}
impl<'a> E50_W<'a> {
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
#[doc = "Reader of field `E51`"]
pub type E51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E51`"]
pub struct E51_W<'a> {
    w: &'a mut W,
}
impl<'a> E51_W<'a> {
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
#[doc = "Reader of field `E52`"]
pub type E52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E52`"]
pub struct E52_W<'a> {
    w: &'a mut W,
}
impl<'a> E52_W<'a> {
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
#[doc = "Reader of field `E53`"]
pub type E53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E53`"]
pub struct E53_W<'a> {
    w: &'a mut W,
}
impl<'a> E53_W<'a> {
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
#[doc = "Reader of field `E54`"]
pub type E54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E54`"]
pub struct E54_W<'a> {
    w: &'a mut W,
}
impl<'a> E54_W<'a> {
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
#[doc = "Reader of field `E55`"]
pub type E55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E55`"]
pub struct E55_W<'a> {
    w: &'a mut W,
}
impl<'a> E55_W<'a> {
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
#[doc = "Reader of field `E56`"]
pub type E56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E56`"]
pub struct E56_W<'a> {
    w: &'a mut W,
}
impl<'a> E56_W<'a> {
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
#[doc = "Reader of field `E57`"]
pub type E57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E57`"]
pub struct E57_W<'a> {
    w: &'a mut W,
}
impl<'a> E57_W<'a> {
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
#[doc = "Reader of field `E58`"]
pub type E58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E58`"]
pub struct E58_W<'a> {
    w: &'a mut W,
}
impl<'a> E58_W<'a> {
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
#[doc = "Reader of field `E59`"]
pub type E59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E59`"]
pub struct E59_W<'a> {
    w: &'a mut W,
}
impl<'a> E59_W<'a> {
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
#[doc = "Reader of field `E60`"]
pub type E60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E60`"]
pub struct E60_W<'a> {
    w: &'a mut W,
}
impl<'a> E60_W<'a> {
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
#[doc = "Reader of field `E61`"]
pub type E61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E61`"]
pub struct E61_W<'a> {
    w: &'a mut W,
}
impl<'a> E61_W<'a> {
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
#[doc = "Reader of field `E62`"]
pub type E62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E62`"]
pub struct E62_W<'a> {
    w: &'a mut W,
}
impl<'a> E62_W<'a> {
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
    #[doc = "Bit 0 - Interrupt Enable of Source for RV_PLIC32"]
    #[inline(always)]
    pub fn e32(&self) -> E32_R {
        E32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC33"]
    #[inline(always)]
    pub fn e33(&self) -> E33_R {
        E33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC34"]
    #[inline(always)]
    pub fn e34(&self) -> E34_R {
        E34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC35"]
    #[inline(always)]
    pub fn e35(&self) -> E35_R {
        E35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC36"]
    #[inline(always)]
    pub fn e36(&self) -> E36_R {
        E36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC37"]
    #[inline(always)]
    pub fn e37(&self) -> E37_R {
        E37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC38"]
    #[inline(always)]
    pub fn e38(&self) -> E38_R {
        E38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC39"]
    #[inline(always)]
    pub fn e39(&self) -> E39_R {
        E39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC40"]
    #[inline(always)]
    pub fn e40(&self) -> E40_R {
        E40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC41"]
    #[inline(always)]
    pub fn e41(&self) -> E41_R {
        E41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC42"]
    #[inline(always)]
    pub fn e42(&self) -> E42_R {
        E42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC43"]
    #[inline(always)]
    pub fn e43(&self) -> E43_R {
        E43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC44"]
    #[inline(always)]
    pub fn e44(&self) -> E44_R {
        E44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC45"]
    #[inline(always)]
    pub fn e45(&self) -> E45_R {
        E45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC46"]
    #[inline(always)]
    pub fn e46(&self) -> E46_R {
        E46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC47"]
    #[inline(always)]
    pub fn e47(&self) -> E47_R {
        E47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC48"]
    #[inline(always)]
    pub fn e48(&self) -> E48_R {
        E48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC49"]
    #[inline(always)]
    pub fn e49(&self) -> E49_R {
        E49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC50"]
    #[inline(always)]
    pub fn e50(&self) -> E50_R {
        E50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC51"]
    #[inline(always)]
    pub fn e51(&self) -> E51_R {
        E51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC52"]
    #[inline(always)]
    pub fn e52(&self) -> E52_R {
        E52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC53"]
    #[inline(always)]
    pub fn e53(&self) -> E53_R {
        E53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC54"]
    #[inline(always)]
    pub fn e54(&self) -> E54_R {
        E54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC55"]
    #[inline(always)]
    pub fn e55(&self) -> E55_R {
        E55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC56"]
    #[inline(always)]
    pub fn e56(&self) -> E56_R {
        E56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC57"]
    #[inline(always)]
    pub fn e57(&self) -> E57_R {
        E57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC58"]
    #[inline(always)]
    pub fn e58(&self) -> E58_R {
        E58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC59"]
    #[inline(always)]
    pub fn e59(&self) -> E59_R {
        E59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC60"]
    #[inline(always)]
    pub fn e60(&self) -> E60_R {
        E60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC61"]
    #[inline(always)]
    pub fn e61(&self) -> E61_R {
        E61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC62"]
    #[inline(always)]
    pub fn e62(&self) -> E62_R {
        E62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable of Source for RV_PLIC32"]
    #[inline(always)]
    pub fn e32(&mut self) -> E32_W {
        E32_W { w: self }
    }
    #[doc = "Bit 1 - for RV_PLIC33"]
    #[inline(always)]
    pub fn e33(&mut self) -> E33_W {
        E33_W { w: self }
    }
    #[doc = "Bit 2 - for RV_PLIC34"]
    #[inline(always)]
    pub fn e34(&mut self) -> E34_W {
        E34_W { w: self }
    }
    #[doc = "Bit 3 - for RV_PLIC35"]
    #[inline(always)]
    pub fn e35(&mut self) -> E35_W {
        E35_W { w: self }
    }
    #[doc = "Bit 4 - for RV_PLIC36"]
    #[inline(always)]
    pub fn e36(&mut self) -> E36_W {
        E36_W { w: self }
    }
    #[doc = "Bit 5 - for RV_PLIC37"]
    #[inline(always)]
    pub fn e37(&mut self) -> E37_W {
        E37_W { w: self }
    }
    #[doc = "Bit 6 - for RV_PLIC38"]
    #[inline(always)]
    pub fn e38(&mut self) -> E38_W {
        E38_W { w: self }
    }
    #[doc = "Bit 7 - for RV_PLIC39"]
    #[inline(always)]
    pub fn e39(&mut self) -> E39_W {
        E39_W { w: self }
    }
    #[doc = "Bit 8 - for RV_PLIC40"]
    #[inline(always)]
    pub fn e40(&mut self) -> E40_W {
        E40_W { w: self }
    }
    #[doc = "Bit 9 - for RV_PLIC41"]
    #[inline(always)]
    pub fn e41(&mut self) -> E41_W {
        E41_W { w: self }
    }
    #[doc = "Bit 10 - for RV_PLIC42"]
    #[inline(always)]
    pub fn e42(&mut self) -> E42_W {
        E42_W { w: self }
    }
    #[doc = "Bit 11 - for RV_PLIC43"]
    #[inline(always)]
    pub fn e43(&mut self) -> E43_W {
        E43_W { w: self }
    }
    #[doc = "Bit 12 - for RV_PLIC44"]
    #[inline(always)]
    pub fn e44(&mut self) -> E44_W {
        E44_W { w: self }
    }
    #[doc = "Bit 13 - for RV_PLIC45"]
    #[inline(always)]
    pub fn e45(&mut self) -> E45_W {
        E45_W { w: self }
    }
    #[doc = "Bit 14 - for RV_PLIC46"]
    #[inline(always)]
    pub fn e46(&mut self) -> E46_W {
        E46_W { w: self }
    }
    #[doc = "Bit 15 - for RV_PLIC47"]
    #[inline(always)]
    pub fn e47(&mut self) -> E47_W {
        E47_W { w: self }
    }
    #[doc = "Bit 16 - for RV_PLIC48"]
    #[inline(always)]
    pub fn e48(&mut self) -> E48_W {
        E48_W { w: self }
    }
    #[doc = "Bit 17 - for RV_PLIC49"]
    #[inline(always)]
    pub fn e49(&mut self) -> E49_W {
        E49_W { w: self }
    }
    #[doc = "Bit 18 - for RV_PLIC50"]
    #[inline(always)]
    pub fn e50(&mut self) -> E50_W {
        E50_W { w: self }
    }
    #[doc = "Bit 19 - for RV_PLIC51"]
    #[inline(always)]
    pub fn e51(&mut self) -> E51_W {
        E51_W { w: self }
    }
    #[doc = "Bit 20 - for RV_PLIC52"]
    #[inline(always)]
    pub fn e52(&mut self) -> E52_W {
        E52_W { w: self }
    }
    #[doc = "Bit 21 - for RV_PLIC53"]
    #[inline(always)]
    pub fn e53(&mut self) -> E53_W {
        E53_W { w: self }
    }
    #[doc = "Bit 22 - for RV_PLIC54"]
    #[inline(always)]
    pub fn e54(&mut self) -> E54_W {
        E54_W { w: self }
    }
    #[doc = "Bit 23 - for RV_PLIC55"]
    #[inline(always)]
    pub fn e55(&mut self) -> E55_W {
        E55_W { w: self }
    }
    #[doc = "Bit 24 - for RV_PLIC56"]
    #[inline(always)]
    pub fn e56(&mut self) -> E56_W {
        E56_W { w: self }
    }
    #[doc = "Bit 25 - for RV_PLIC57"]
    #[inline(always)]
    pub fn e57(&mut self) -> E57_W {
        E57_W { w: self }
    }
    #[doc = "Bit 26 - for RV_PLIC58"]
    #[inline(always)]
    pub fn e58(&mut self) -> E58_W {
        E58_W { w: self }
    }
    #[doc = "Bit 27 - for RV_PLIC59"]
    #[inline(always)]
    pub fn e59(&mut self) -> E59_W {
        E59_W { w: self }
    }
    #[doc = "Bit 28 - for RV_PLIC60"]
    #[inline(always)]
    pub fn e60(&mut self) -> E60_W {
        E60_W { w: self }
    }
    #[doc = "Bit 29 - for RV_PLIC61"]
    #[inline(always)]
    pub fn e61(&mut self) -> E61_W {
        E61_W { w: self }
    }
    #[doc = "Bit 30 - for RV_PLIC62"]
    #[inline(always)]
    pub fn e62(&mut self) -> E62_W {
        E62_W { w: self }
    }
}
