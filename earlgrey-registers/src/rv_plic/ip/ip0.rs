#[doc = "Reader of register IP0"]
pub type R = crate::R<u32, super::IP0>;
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1`"]
pub type P1_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2`"]
pub type P2_R = crate::R<bool, bool>;
#[doc = "Reader of field `P3`"]
pub type P3_R = crate::R<bool, bool>;
#[doc = "Reader of field `P4`"]
pub type P4_R = crate::R<bool, bool>;
#[doc = "Reader of field `P5`"]
pub type P5_R = crate::R<bool, bool>;
#[doc = "Reader of field `P6`"]
pub type P6_R = crate::R<bool, bool>;
#[doc = "Reader of field `P7`"]
pub type P7_R = crate::R<bool, bool>;
#[doc = "Reader of field `P8`"]
pub type P8_R = crate::R<bool, bool>;
#[doc = "Reader of field `P9`"]
pub type P9_R = crate::R<bool, bool>;
#[doc = "Reader of field `P10`"]
pub type P10_R = crate::R<bool, bool>;
#[doc = "Reader of field `P11`"]
pub type P11_R = crate::R<bool, bool>;
#[doc = "Reader of field `P12`"]
pub type P12_R = crate::R<bool, bool>;
#[doc = "Reader of field `P13`"]
pub type P13_R = crate::R<bool, bool>;
#[doc = "Reader of field `P14`"]
pub type P14_R = crate::R<bool, bool>;
#[doc = "Reader of field `P15`"]
pub type P15_R = crate::R<bool, bool>;
#[doc = "Reader of field `P16`"]
pub type P16_R = crate::R<bool, bool>;
#[doc = "Reader of field `P17`"]
pub type P17_R = crate::R<bool, bool>;
#[doc = "Reader of field `P18`"]
pub type P18_R = crate::R<bool, bool>;
#[doc = "Reader of field `P19`"]
pub type P19_R = crate::R<bool, bool>;
#[doc = "Reader of field `P20`"]
pub type P20_R = crate::R<bool, bool>;
#[doc = "Reader of field `P21`"]
pub type P21_R = crate::R<bool, bool>;
#[doc = "Reader of field `P22`"]
pub type P22_R = crate::R<bool, bool>;
#[doc = "Reader of field `P23`"]
pub type P23_R = crate::R<bool, bool>;
#[doc = "Reader of field `P24`"]
pub type P24_R = crate::R<bool, bool>;
#[doc = "Reader of field `P25`"]
pub type P25_R = crate::R<bool, bool>;
#[doc = "Reader of field `P26`"]
pub type P26_R = crate::R<bool, bool>;
#[doc = "Reader of field `P27`"]
pub type P27_R = crate::R<bool, bool>;
#[doc = "Reader of field `P28`"]
pub type P28_R = crate::R<bool, bool>;
#[doc = "Reader of field `P29`"]
pub type P29_R = crate::R<bool, bool>;
#[doc = "Reader of field `P30`"]
pub type P30_R = crate::R<bool, bool>;
#[doc = "Reader of field `P31`"]
pub type P31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Pending of Source for RV_PLIC0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for RV_PLIC1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for RV_PLIC2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for RV_PLIC3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for RV_PLIC4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for RV_PLIC5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for RV_PLIC6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for RV_PLIC7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - for RV_PLIC8"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - for RV_PLIC9"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - for RV_PLIC10"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for RV_PLIC11"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - for RV_PLIC12"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - for RV_PLIC13"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - for RV_PLIC14"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - for RV_PLIC15"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for RV_PLIC16"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for RV_PLIC17"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for RV_PLIC18"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for RV_PLIC19"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - for RV_PLIC20"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - for RV_PLIC21"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for RV_PLIC22"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for RV_PLIC23"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - for RV_PLIC24"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for RV_PLIC25"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for RV_PLIC26"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for RV_PLIC27"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - for RV_PLIC28"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - for RV_PLIC29"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - for RV_PLIC30"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - for RV_PLIC31"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
