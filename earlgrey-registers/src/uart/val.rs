#[doc = "Reader of register VAL"]
pub type R = crate::R<u32, super::VAL>;
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Last 16 oversampled values of RX. Most recent bit is bit 0, oldest 15."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xffff) as u16)
    }
}
