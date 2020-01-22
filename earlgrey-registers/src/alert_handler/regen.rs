#[doc = "Reader of register REGEN"]
pub type R = crate::R<u8, super::REGEN>;
#[doc = "Writer for register REGEN"]
pub type W = crate::W<u8, super::REGEN>;
#[doc = "Register REGEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::REGEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
impl R {}
impl W {}
