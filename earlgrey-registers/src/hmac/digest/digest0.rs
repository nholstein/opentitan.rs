#[doc = "Reader of register DIGEST0"]
pub type R = crate::R<u32, super::DIGEST0>;
#[doc = "Reader of field `digest0`"]
pub type DIGEST0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC0"]
    #[inline(always)]
    pub fn digest0(&self) -> DIGEST0_R {
        DIGEST0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
