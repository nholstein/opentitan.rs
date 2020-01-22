#[doc = "Reader of register PERIPH_INSEL0"]
pub type R = crate::R<u32, super::PERIPH_INSEL0>;
#[doc = "Writer for register PERIPH_INSEL0"]
pub type W = crate::W<u32, super::PERIPH_INSEL0>;
#[doc = "Register PERIPH_INSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIPH_INSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN4`"]
pub struct IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN0"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - for IN1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - for IN2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - for IN3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - for IN4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: tie constantly to zero, 1: tie constantly to 1. >=2: MIO pads (i.e., add 2 to the native MIO pad index). for IN0"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bits 6:11 - for IN1"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bits 12:17 - for IN2"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bits 18:23 - for IN3"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bits 24:29 - for IN4"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W {
        IN4_W { w: self }
    }
}
