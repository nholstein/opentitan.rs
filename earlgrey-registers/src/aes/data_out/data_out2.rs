#[doc = "Reader of register DATA_OUT2"]
pub type R = crate::R<u32, super::DATA_OUT2>;
#[doc = "Reader of field `data_out2`"]
pub type DATA_OUT2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data for DATA_OUT2"]
    #[inline(always)]
    pub fn data_out2(&self) -> DATA_OUT2_R {
        DATA_OUT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
