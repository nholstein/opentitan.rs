#[doc = "Reader of register IP1"]
pub type R = crate::R<u32, super::IP1>;
#[doc = "Reader of field `P32`"]
pub type P32_R = crate::R<bool, bool>;
#[doc = "Reader of field `P33`"]
pub type P33_R = crate::R<bool, bool>;
#[doc = "Reader of field `P34`"]
pub type P34_R = crate::R<bool, bool>;
#[doc = "Reader of field `P35`"]
pub type P35_R = crate::R<bool, bool>;
#[doc = "Reader of field `P36`"]
pub type P36_R = crate::R<bool, bool>;
#[doc = "Reader of field `P37`"]
pub type P37_R = crate::R<bool, bool>;
#[doc = "Reader of field `P38`"]
pub type P38_R = crate::R<bool, bool>;
#[doc = "Reader of field `P39`"]
pub type P39_R = crate::R<bool, bool>;
#[doc = "Reader of field `P40`"]
pub type P40_R = crate::R<bool, bool>;
#[doc = "Reader of field `P41`"]
pub type P41_R = crate::R<bool, bool>;
#[doc = "Reader of field `P42`"]
pub type P42_R = crate::R<bool, bool>;
#[doc = "Reader of field `P43`"]
pub type P43_R = crate::R<bool, bool>;
#[doc = "Reader of field `P44`"]
pub type P44_R = crate::R<bool, bool>;
#[doc = "Reader of field `P45`"]
pub type P45_R = crate::R<bool, bool>;
#[doc = "Reader of field `P46`"]
pub type P46_R = crate::R<bool, bool>;
#[doc = "Reader of field `P47`"]
pub type P47_R = crate::R<bool, bool>;
#[doc = "Reader of field `P48`"]
pub type P48_R = crate::R<bool, bool>;
#[doc = "Reader of field `P49`"]
pub type P49_R = crate::R<bool, bool>;
#[doc = "Reader of field `P50`"]
pub type P50_R = crate::R<bool, bool>;
#[doc = "Reader of field `P51`"]
pub type P51_R = crate::R<bool, bool>;
#[doc = "Reader of field `P52`"]
pub type P52_R = crate::R<bool, bool>;
#[doc = "Reader of field `P53`"]
pub type P53_R = crate::R<bool, bool>;
#[doc = "Reader of field `P54`"]
pub type P54_R = crate::R<bool, bool>;
#[doc = "Reader of field `P55`"]
pub type P55_R = crate::R<bool, bool>;
#[doc = "Reader of field `P56`"]
pub type P56_R = crate::R<bool, bool>;
#[doc = "Reader of field `P57`"]
pub type P57_R = crate::R<bool, bool>;
#[doc = "Reader of field `P58`"]
pub type P58_R = crate::R<bool, bool>;
#[doc = "Reader of field `P59`"]
pub type P59_R = crate::R<bool, bool>;
#[doc = "Reader of field `P60`"]
pub type P60_R = crate::R<bool, bool>;
#[doc = "Reader of field `P61`"]
pub type P61_R = crate::R<bool, bool>;
#[doc = "Reader of field `P62`"]
pub type P62_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Pending of Source for RV_PLIC32"]
    #[inline(always)]
    pub fn p32(&self) -> P32_R {
        P32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC33"]
    #[inline(always)]
    pub fn p33(&self) -> P33_R {
        P33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC34"]
    #[inline(always)]
    pub fn p34(&self) -> P34_R {
        P34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC35"]
    #[inline(always)]
    pub fn p35(&self) -> P35_R {
        P35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC36"]
    #[inline(always)]
    pub fn p36(&self) -> P36_R {
        P36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC37"]
    #[inline(always)]
    pub fn p37(&self) -> P37_R {
        P37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC38"]
    #[inline(always)]
    pub fn p38(&self) -> P38_R {
        P38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC39"]
    #[inline(always)]
    pub fn p39(&self) -> P39_R {
        P39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC40"]
    #[inline(always)]
    pub fn p40(&self) -> P40_R {
        P40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC41"]
    #[inline(always)]
    pub fn p41(&self) -> P41_R {
        P41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC42"]
    #[inline(always)]
    pub fn p42(&self) -> P42_R {
        P42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC43"]
    #[inline(always)]
    pub fn p43(&self) -> P43_R {
        P43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC44"]
    #[inline(always)]
    pub fn p44(&self) -> P44_R {
        P44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC45"]
    #[inline(always)]
    pub fn p45(&self) -> P45_R {
        P45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC46"]
    #[inline(always)]
    pub fn p46(&self) -> P46_R {
        P46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC47"]
    #[inline(always)]
    pub fn p47(&self) -> P47_R {
        P47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC48"]
    #[inline(always)]
    pub fn p48(&self) -> P48_R {
        P48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC49"]
    #[inline(always)]
    pub fn p49(&self) -> P49_R {
        P49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC50"]
    #[inline(always)]
    pub fn p50(&self) -> P50_R {
        P50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC51"]
    #[inline(always)]
    pub fn p51(&self) -> P51_R {
        P51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC52"]
    #[inline(always)]
    pub fn p52(&self) -> P52_R {
        P52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC53"]
    #[inline(always)]
    pub fn p53(&self) -> P53_R {
        P53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC54"]
    #[inline(always)]
    pub fn p54(&self) -> P54_R {
        P54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC55"]
    #[inline(always)]
    pub fn p55(&self) -> P55_R {
        P55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC56"]
    #[inline(always)]
    pub fn p56(&self) -> P56_R {
        P56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC57"]
    #[inline(always)]
    pub fn p57(&self) -> P57_R {
        P57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC58"]
    #[inline(always)]
    pub fn p58(&self) -> P58_R {
        P58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC59"]
    #[inline(always)]
    pub fn p59(&self) -> P59_R {
        P59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC60"]
    #[inline(always)]
    pub fn p60(&self) -> P60_R {
        P60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC61"]
    #[inline(always)]
    pub fn p61(&self) -> P61_R {
        P61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC62"]
    #[inline(always)]
    pub fn p62(&self) -> P62_R {
        P62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
