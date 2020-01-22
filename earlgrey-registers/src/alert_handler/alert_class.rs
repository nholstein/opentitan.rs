#[doc = "Reader of register ALERT_CLASS"]
pub type R = crate::R<u32, super::ALERT_CLASS>;
#[doc = "Writer for register ALERT_CLASS"]
pub type W = crate::W<u32, super::ALERT_CLASS>;
#[doc = "Register ALERT_CLASS `reset()`'s with value 0"]
impl crate::ResetValue for super::ALERT_CLASS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLASS_A0`"]
pub type CLASS_A0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_A0`"]
pub struct CLASS_A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CLASS_A1`"]
pub type CLASS_A1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_A1`"]
pub struct CLASS_A1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_A1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLASS_A2`"]
pub type CLASS_A2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_A2`"]
pub struct CLASS_A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLASS_A3`"]
pub type CLASS_A3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_A3`"]
pub struct CLASS_A3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_A3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Classification for alert0"]
    #[inline(always)]
    pub fn class_a0(&self) -> CLASS_A0_R {
        CLASS_A0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - for alert1"]
    #[inline(always)]
    pub fn class_a1(&self) -> CLASS_A1_R {
        CLASS_A1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - for alert2"]
    #[inline(always)]
    pub fn class_a2(&self) -> CLASS_A2_R {
        CLASS_A2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - for alert3"]
    #[inline(always)]
    pub fn class_a3(&self) -> CLASS_A3_R {
        CLASS_A3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Classification for alert0"]
    #[inline(always)]
    pub fn class_a0(&mut self) -> CLASS_A0_W {
        CLASS_A0_W { w: self }
    }
    #[doc = "Bits 2:3 - for alert1"]
    #[inline(always)]
    pub fn class_a1(&mut self) -> CLASS_A1_W {
        CLASS_A1_W { w: self }
    }
    #[doc = "Bits 4:5 - for alert2"]
    #[inline(always)]
    pub fn class_a2(&mut self) -> CLASS_A2_W {
        CLASS_A2_W { w: self }
    }
    #[doc = "Bits 6:7 - for alert3"]
    #[inline(always)]
    pub fn class_a3(&mut self) -> CLASS_A3_W {
        CLASS_A3_W { w: self }
    }
}
