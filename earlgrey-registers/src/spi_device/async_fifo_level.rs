#[doc = "Reader of register ASYNC_FIFO_LEVEL"]
pub type R = crate::R<u32, super::ASYNC_FIFO_LEVEL>;
#[doc = "Reader of field `rxlvl`"]
pub type RXLVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `txlvl`"]
pub type TXLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Async FIFO level. This value shows the number of available entry in RX Async FIFO."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX Async FIFO level. This value shows the number of available entry in TX Async FIFO. If the software writes message into SRAM FIFO and update FIFO write pointer but no clock from the host is given, the data stuck at this async fifo waiting host toggles SCK. This value represents the number of bytes."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
