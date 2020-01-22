#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `rxf_full`"]
pub type RXF_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `rxf_empty`"]
pub type RXF_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `txf_full`"]
pub type TXF_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `txf_empty`"]
pub type TXF_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `abort_done`"]
pub type ABORT_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `csb`"]
pub type CSB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RX FIFO full"]
    #[inline(always)]
    pub fn rxf_full(&self) -> RXF_FULL_R {
        RXF_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO empty"]
    #[inline(always)]
    pub fn rxf_empty(&self) -> RXF_EMPTY_R {
        RXF_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX FIFO full"]
    #[inline(always)]
    pub fn txf_full(&self) -> TXF_FULL_R {
        TXF_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FIFO empty"]
    #[inline(always)]
    pub fn txf_empty(&self) -> TXF_EMPTY_R {
        TXF_EMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Abort process is completed"]
    #[inline(always)]
    pub fn abort_done(&self) -> ABORT_DONE_R {
        ABORT_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Direct input of CSb signal"]
    #[inline(always)]
    pub fn csb(&self) -> CSB_R {
        CSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
