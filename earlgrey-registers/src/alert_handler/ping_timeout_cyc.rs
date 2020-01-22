#[doc = "Reader of register PING_TIMEOUT_CYC"]
pub type R = crate::R<u32, super::PING_TIMEOUT_CYC>;
#[doc = "Writer for register PING_TIMEOUT_CYC"]
pub type W = crate::W<u32, super::PING_TIMEOUT_CYC>;
#[doc = "Register PING_TIMEOUT_CYC `reset()`'s with value 0x20"]
impl crate::ResetValue for super::PING_TIMEOUT_CYC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
impl R {}
impl W {}
