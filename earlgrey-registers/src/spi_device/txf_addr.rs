#[doc = "Reader of register TXF_ADDR"]
pub type R = crate::R<u32, super::TXF_ADDR>;
#[doc = "Writer for register TXF_ADDR"]
pub type W = crate::W<u32, super::TXF_ADDR>;
#[doc = "Register TXF_ADDR `reset()`'s with value 0x03fc_0200"]
impl crate::ResetValue for super::TXF_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03fc_0200
    }
}
#[doc = "Reader of field `base`"]
pub type BASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `base`"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `limit`"]
pub type LIMIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `limit`"]
pub struct LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Base offset in bytes in the SRAM. Lower 2 bits are ignored."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Limit offset in bytes in the SRAM. Lower 2 bits are ignored."]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Base offset in bytes in the SRAM. Lower 2 bits are ignored."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
    #[doc = "Bits 16:31 - Limit offset in bytes in the SRAM. Lower 2 bits are ignored."]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W {
        LIMIT_W { w: self }
    }
}
