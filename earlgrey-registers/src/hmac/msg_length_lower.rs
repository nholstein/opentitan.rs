#[doc = "Reader of register MSG_LENGTH_LOWER"]
pub type R = crate::R<u32, super::MSG_LENGTH_LOWER>;
#[doc = "Reader of field `v`"]
pub type V_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Length \\[31:0\\]"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
