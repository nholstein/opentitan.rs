#[doc = "Reader of register DIGEST6"]
pub type R = crate::R<u32, super::DIGEST6>;
#[doc = "Reader of field `digest6`"]
pub type DIGEST6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC6"]
    #[inline(always)]
    pub fn digest6(&self) -> DIGEST6_R {
        DIGEST6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
