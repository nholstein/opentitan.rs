#[doc = "Reader of register DATA_OUT3"]
pub type R = crate::R<u32, super::DATA_OUT3>;
#[doc = "Reader of field `data_out3`"]
pub type DATA_OUT3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data for DATA_OUT3"]
    #[inline(always)]
    pub fn data_out3(&self) -> DATA_OUT3_R {
        DATA_OUT3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
