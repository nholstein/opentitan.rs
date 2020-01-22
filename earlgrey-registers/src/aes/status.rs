#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTPUT_VALID`"]
pub type OUTPUT_VALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `INPUT_READY`"]
pub type INPUT_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - The AES unit is idle (0) or busy (1)."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The AES unit is not stalled (0) or stalled (1) because there is previous output data that must be read by the processor before the AES unit can overwrite this data."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The AES unit has no valid output (0) or has valid output data (1)."]
    #[inline(always)]
    pub fn output_valid(&self) -> OUTPUT_VALID_R {
        OUTPUT_VALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The AES unit is ready (1) to receive new data input via the DATA_IN registers or the present values in the DATA_IN registers have not yet been loaded into the module (0)."]
    #[inline(always)]
    pub fn input_ready(&self) -> INPUT_READY_R {
        INPUT_READY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
