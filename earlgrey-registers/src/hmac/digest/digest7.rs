#[doc = "Reader of register DIGEST7"]
pub type R = crate::R<u32, super::DIGEST7>;
#[doc = "Reader of field `digest7`"]
pub type DIGEST7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC7"]
    #[inline(always)]
    pub fn digest7(&self) -> DIGEST7_R {
        DIGEST7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
