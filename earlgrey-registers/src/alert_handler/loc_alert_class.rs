#[doc = "Reader of register LOC_ALERT_CLASS"]
pub type R = crate::R<u32, super::LOC_ALERT_CLASS>;
#[doc = "Writer for register LOC_ALERT_CLASS"]
pub type W = crate::W<u32, super::LOC_ALERT_CLASS>;
#[doc = "Register LOC_ALERT_CLASS `reset()`'s with value 0"]
impl crate::ResetValue for super::LOC_ALERT_CLASS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLASS_LA0`"]
pub type CLASS_LA0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_LA0`"]
pub struct CLASS_LA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_LA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CLASS_LA1`"]
pub type CLASS_LA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_LA1`"]
pub struct CLASS_LA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_LA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLASS_LA2`"]
pub type CLASS_LA2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_LA2`"]
pub struct CLASS_LA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_LA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLASS_LA3`"]
pub type CLASS_LA3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLASS_LA3`"]
pub struct CLASS_LA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLASS_LA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Classification for local alert0"]
    #[inline(always)]
    pub fn class_la0(&self) -> CLASS_LA0_R {
        CLASS_LA0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - for local alert1"]
    #[inline(always)]
    pub fn class_la1(&self) -> CLASS_LA1_R {
        CLASS_LA1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - for local alert2"]
    #[inline(always)]
    pub fn class_la2(&self) -> CLASS_LA2_R {
        CLASS_LA2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - for local alert3"]
    #[inline(always)]
    pub fn class_la3(&self) -> CLASS_LA3_R {
        CLASS_LA3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Classification for local alert0"]
    #[inline(always)]
    pub fn class_la0(&mut self) -> CLASS_LA0_W {
        CLASS_LA0_W { w: self }
    }
    #[doc = "Bits 2:3 - for local alert1"]
    #[inline(always)]
    pub fn class_la1(&mut self) -> CLASS_LA1_W {
        CLASS_LA1_W { w: self }
    }
    #[doc = "Bits 4:5 - for local alert2"]
    #[inline(always)]
    pub fn class_la2(&mut self) -> CLASS_LA2_W {
        CLASS_LA2_W { w: self }
    }
    #[doc = "Bits 6:7 - for local alert3"]
    #[inline(always)]
    pub fn class_la3(&mut self) -> CLASS_LA3_W {
        CLASS_LA3_W { w: self }
    }
}
