#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
}
#[doc = "Interrupt State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_state](intr_state) module"]
pub type INTR_STATE = crate::Reg<u32, _INTR_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_STATE;
#[doc = "`read()` method returns [intr_state::R](intr_state::R) reader structure"]
impl crate::Readable for INTR_STATE {}
#[doc = "`write(|w| ..)` method takes [intr_state::W](intr_state::W) writer structure"]
impl crate::Writable for INTR_STATE {}
#[doc = "Interrupt State Register"]
pub mod intr_state;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_enable](intr_enable) module"]
pub type INTR_ENABLE = crate::Reg<u32, _INTR_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_ENABLE;
#[doc = "`read()` method returns [intr_enable::R](intr_enable::R) reader structure"]
impl crate::Readable for INTR_ENABLE {}
#[doc = "`write(|w| ..)` method takes [intr_enable::W](intr_enable::W) writer structure"]
impl crate::Writable for INTR_ENABLE {}
#[doc = "Interrupt Enable Register"]
pub mod intr_enable;
#[doc = "Interrupt Test Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_test](intr_test) module"]
pub type INTR_TEST = crate::Reg<u32, _INTR_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TEST;
#[doc = "`write(|w| ..)` method takes [intr_test::W](intr_test::W) writer structure"]
impl crate::Writable for INTR_TEST {}
#[doc = "Interrupt Test Register"]
pub mod intr_test;
