#[doc = "Reader of register FIFO_STATUS"]
pub type R = crate::R<u32, super::FIFO_STATUS>;
#[doc = "Reader of field `TXLVL`"]
pub type TXLVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXLVL`"]
pub type RXLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Current fill level of TX fifo"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Current fill level of RX fifo"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
