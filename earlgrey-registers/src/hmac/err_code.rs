#[doc = "Reader of register ERR_CODE"]
pub type R = crate::R<u32, super::ERR_CODE>;
#[doc = "Reader of field `err_code`"]
pub type ERR_CODE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - If error interrupt occurs, this register has information of error cause. Please take a look at `hw/ip/hmac/rtl/hmac_pkg.sv:err_code_e enum type."]
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
