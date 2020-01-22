#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `fifo_empty`"]
pub type FIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `fifo_full`"]
pub type FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `fifo_depth`"]
pub type FIFO_DEPTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO full. Data written to the FIFO whilst it is full will cause back-pressure on the interconnect"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - FIFO entry count."]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
