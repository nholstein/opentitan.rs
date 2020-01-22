#[doc = "Reader of register RXF_PTR"]
pub type R = crate::R<u32, super::RXF_PTR>;
#[doc = "Writer for register RXF_PTR"]
pub type W = crate::W<u32, super::RXF_PTR>;
#[doc = "Register RXF_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXF_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPTR`"]
pub type RPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RPTR`"]
pub struct RPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WPTR`"]
pub type WPTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read pointer. bit x is for phase bit. check circular fifo description"]
    #[inline(always)]
    pub fn rptr(&self) -> RPTR_R {
        RPTR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write pointer. Bit x is phase bit."]
    #[inline(always)]
    pub fn wptr(&self) -> WPTR_R {
        WPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read pointer. bit x is for phase bit. check circular fifo description"]
    #[inline(always)]
    pub fn rptr(&mut self) -> RPTR_W {
        RPTR_W { w: self }
    }
}
