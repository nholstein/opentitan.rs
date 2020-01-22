#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `prescale`"]
pub type PRESCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `prescale`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `step`"]
pub type STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `step`"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Prescaler to generate tick"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - Incremental value for each tick"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Prescaler to generate tick"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bits 16:23 - Incremental value for each tick"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
}
