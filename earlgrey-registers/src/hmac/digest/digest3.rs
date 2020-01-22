#[doc = "Reader of register DIGEST3"]
pub type R = crate::R<u32, super::DIGEST3>;
#[doc = "Reader of field `digest3`"]
pub type DIGEST3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC3"]
    #[inline(always)]
    pub fn digest3(&self) -> DIGEST3_R {
        DIGEST3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
