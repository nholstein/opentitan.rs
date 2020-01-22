#[doc = "Reader of register DIGEST5"]
pub type R = crate::R<u32, super::DIGEST5>;
#[doc = "Reader of field `digest5`"]
pub type DIGEST5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC5"]
    #[inline(always)]
    pub fn digest5(&self) -> DIGEST5_R {
        DIGEST5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
