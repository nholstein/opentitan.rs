#[doc = "Reader of register DATA_OUT0"]
pub type R = crate::R<u32, super::DATA_OUT0>;
#[doc = "Reader of field `data_out0`"]
pub type DATA_OUT0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data for DATA_OUT0"]
    #[inline(always)]
    pub fn data_out0(&self) -> DATA_OUT0_R {
        DATA_OUT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
