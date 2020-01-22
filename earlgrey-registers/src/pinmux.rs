#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Register write enable for all control registers."]
    pub regen: REGEN,
    #[doc = "0x04 - Mux select for peripheral inputs."]
    pub periph_insel: PERIPH_INSEL,
    #[doc = "0x20 - Mux select for MIO outputs."]
    pub mio_outsel: MIO_OUTSEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PERIPH_INSEL {
    #[doc = "0x00 - Mux select for peripheral inputs."]
    pub periph_insel0: self::periph_insel::PERIPH_INSEL0,
    #[doc = "0x04 - Mux select for peripheral inputs."]
    pub periph_insel1: self::periph_insel::PERIPH_INSEL1,
    #[doc = "0x08 - Mux select for peripheral inputs."]
    pub periph_insel2: self::periph_insel::PERIPH_INSEL2,
    #[doc = "0x0c - Mux select for peripheral inputs."]
    pub periph_insel3: self::periph_insel::PERIPH_INSEL3,
    #[doc = "0x10 - Mux select for peripheral inputs."]
    pub periph_insel4: self::periph_insel::PERIPH_INSEL4,
    #[doc = "0x14 - Mux select for peripheral inputs."]
    pub periph_insel5: self::periph_insel::PERIPH_INSEL5,
    #[doc = "0x18 - Mux select for peripheral inputs."]
    pub periph_insel6: self::periph_insel::PERIPH_INSEL6,
}
#[doc = r"Register block"]
#[doc = "Mux select for peripheral inputs."]
pub mod periph_insel;
#[doc = r"Register block"]
#[repr(C)]
pub struct MIO_OUTSEL {
    #[doc = "0x00 - Mux select for MIO outputs."]
    pub mio_outsel0: self::mio_outsel::MIO_OUTSEL0,
    #[doc = "0x04 - Mux select for MIO outputs."]
    pub mio_outsel1: self::mio_outsel::MIO_OUTSEL1,
    #[doc = "0x08 - Mux select for MIO outputs."]
    pub mio_outsel2: self::mio_outsel::MIO_OUTSEL2,
    #[doc = "0x0c - Mux select for MIO outputs."]
    pub mio_outsel3: self::mio_outsel::MIO_OUTSEL3,
    #[doc = "0x10 - Mux select for MIO outputs."]
    pub mio_outsel4: self::mio_outsel::MIO_OUTSEL4,
    #[doc = "0x14 - Mux select for MIO outputs."]
    pub mio_outsel5: self::mio_outsel::MIO_OUTSEL5,
    #[doc = "0x18 - Mux select for MIO outputs."]
    pub mio_outsel6: self::mio_outsel::MIO_OUTSEL6,
}
#[doc = r"Register block"]
#[doc = "Mux select for MIO outputs."]
pub mod mio_outsel;
#[doc = "Register write enable for all control registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regen](regen) module"]
pub type REGEN = crate::Reg<u32, _REGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGEN;
#[doc = "`read()` method returns [regen::R](regen::R) reader structure"]
impl crate::Readable for REGEN {}
#[doc = "`write(|w| ..)` method takes [regen::W](regen::W) writer structure"]
impl crate::Writable for REGEN {}
#[doc = "Register write enable for all control registers."]
pub mod regen;
