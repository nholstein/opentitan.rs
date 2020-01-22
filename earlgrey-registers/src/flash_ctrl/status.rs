#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `rd_full`"]
pub type RD_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `rd_empty`"]
pub type RD_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `prog_full`"]
pub type PROG_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `prog_empty`"]
pub type PROG_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `init_wip`"]
pub type INIT_WIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `error_page`"]
pub type ERROR_PAGE_R = crate::R<u16, u16>;
#[doc = "Reader of field `error_bank`"]
pub type ERROR_BANK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Flash read fifo full, software must consume data"]
    #[inline(always)]
    pub fn rd_full(&self) -> RD_FULL_R {
        RD_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash read fifo empty"]
    #[inline(always)]
    pub fn rd_empty(&self) -> RD_EMPTY_R {
        RD_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash program fifo full"]
    #[inline(always)]
    pub fn prog_full(&self) -> PROG_FULL_R {
        PROG_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash program fifo empty, software must provide data"]
    #[inline(always)]
    pub fn prog_empty(&self) -> PROG_EMPTY_R {
        PROG_EMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash controller undergoing init"]
    #[inline(always)]
    pub fn init_wip(&self) -> INIT_WIP_R {
        INIT_WIP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:16 - Flash controller error page."]
    #[inline(always)]
    pub fn error_page(&self) -> ERROR_PAGE_R {
        ERROR_PAGE_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 17 - Flash controller error bank."]
    #[inline(always)]
    pub fn error_bank(&self) -> ERROR_BANK_R {
        ERROR_BANK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
