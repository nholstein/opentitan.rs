#[doc = "Reader of register DATA_OUT1"]
pub type R = crate::R<u32, super::DATA_OUT1>;
#[doc = "Reader of field `data_out1`"]
pub type DATA_OUT1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data for DATA_OUT1"]
    #[inline(always)]
    pub fn data_out1(&self) -> DATA_OUT1_R {
        DATA_OUT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
