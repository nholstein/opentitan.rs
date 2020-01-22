#[doc = "Reader of register DIGEST1"]
pub type R = crate::R<u32, super::DIGEST1>;
#[doc = "Reader of field `digest1`"]
pub type DIGEST1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit chunk of 256-bit Digest for HMAC1"]
    #[inline(always)]
    pub fn digest1(&self) -> DIGEST1_R {
        DIGEST1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
