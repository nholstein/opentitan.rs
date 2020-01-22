#[doc = "Reader of register DIGEST4"]
pub type R = crate::R<u32, super::DIGEST4>;
#[doc = "Reader of field `digest4`"]
pub type DIGEST4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC4"]
    #[inline(always)]
    pub fn digest4(&self) -> DIGEST4_R {
        DIGEST4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
