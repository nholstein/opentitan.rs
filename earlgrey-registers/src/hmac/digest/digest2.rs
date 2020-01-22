#[doc = "Reader of register DIGEST2"]
pub type R = crate::R<u32, super::DIGEST2>;
#[doc = "Reader of field `digest2`"]
pub type DIGEST2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC2"]
    #[inline(always)]
    pub fn digest2(&self) -> DIGEST2_R {
        DIGEST2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
