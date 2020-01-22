#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Configuration for Hart 0"]
    pub cfg0: CFG0,
    #[doc = "0x104 - Timer value Lower"]
    pub timer_v_lower0: TIMER_V_LOWER0,
    #[doc = "0x108 - Timer value Upper"]
    pub timer_v_upper0: TIMER_V_UPPER0,
    #[doc = "0x10c - Timer value Lower"]
    pub compare_lower0_0: COMPARE_LOWER0_0,
    #[doc = "0x110 - Timer value Upper"]
    pub compare_upper0_0: COMPARE_UPPER0_0,
    #[doc = "0x114 - Interrupt Enable"]
    pub intr_enable0: INTR_ENABLE0,
    #[doc = "0x118 - Interrupt Status"]
    pub intr_state0: INTR_STATE0,
    #[doc = "0x11c - Interrupt test register"]
    pub intr_test0: INTR_TEST0,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Configuration for Hart 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "Configuration for Hart 0"]
pub mod cfg0;
#[doc = "Timer value Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_v_lower0](timer_v_lower0) module"]
pub type TIMER_V_LOWER0 = crate::Reg<u32, _TIMER_V_LOWER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_V_LOWER0;
#[doc = "`read()` method returns [timer_v_lower0::R](timer_v_lower0::R) reader structure"]
impl crate::Readable for TIMER_V_LOWER0 {}
#[doc = "`write(|w| ..)` method takes [timer_v_lower0::W](timer_v_lower0::W) writer structure"]
impl crate::Writable for TIMER_V_LOWER0 {}
#[doc = "Timer value Lower"]
pub mod timer_v_lower0;
#[doc = "Timer value Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_v_upper0](timer_v_upper0) module"]
pub type TIMER_V_UPPER0 = crate::Reg<u32, _TIMER_V_UPPER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_V_UPPER0;
#[doc = "`read()` method returns [timer_v_upper0::R](timer_v_upper0::R) reader structure"]
impl crate::Readable for TIMER_V_UPPER0 {}
#[doc = "`write(|w| ..)` method takes [timer_v_upper0::W](timer_v_upper0::W) writer structure"]
impl crate::Writable for TIMER_V_UPPER0 {}
#[doc = "Timer value Upper"]
pub mod timer_v_upper0;
#[doc = "Timer value Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compare_lower0_0](compare_lower0_0) module"]
pub type COMPARE_LOWER0_0 = crate::Reg<u32, _COMPARE_LOWER0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPARE_LOWER0_0;
#[doc = "`read()` method returns [compare_lower0_0::R](compare_lower0_0::R) reader structure"]
impl crate::Readable for COMPARE_LOWER0_0 {}
#[doc = "`write(|w| ..)` method takes [compare_lower0_0::W](compare_lower0_0::W) writer structure"]
impl crate::Writable for COMPARE_LOWER0_0 {}
#[doc = "Timer value Lower"]
pub mod compare_lower0_0;
#[doc = "Timer value Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compare_upper0_0](compare_upper0_0) module"]
pub type COMPARE_UPPER0_0 = crate::Reg<u32, _COMPARE_UPPER0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPARE_UPPER0_0;
#[doc = "`read()` method returns [compare_upper0_0::R](compare_upper0_0::R) reader structure"]
impl crate::Readable for COMPARE_UPPER0_0 {}
#[doc = "`write(|w| ..)` method takes [compare_upper0_0::W](compare_upper0_0::W) writer structure"]
impl crate::Writable for COMPARE_UPPER0_0 {}
#[doc = "Timer value Upper"]
pub mod compare_upper0_0;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_enable0](intr_enable0) module"]
pub type INTR_ENABLE0 = crate::Reg<u32, _INTR_ENABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_ENABLE0;
#[doc = "`read()` method returns [intr_enable0::R](intr_enable0::R) reader structure"]
impl crate::Readable for INTR_ENABLE0 {}
#[doc = "`write(|w| ..)` method takes [intr_enable0::W](intr_enable0::W) writer structure"]
impl crate::Writable for INTR_ENABLE0 {}
#[doc = "Interrupt Enable"]
pub mod intr_enable0;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_state0](intr_state0) module"]
pub type INTR_STATE0 = crate::Reg<u32, _INTR_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_STATE0;
#[doc = "`read()` method returns [intr_state0::R](intr_state0::R) reader structure"]
impl crate::Readable for INTR_STATE0 {}
#[doc = "`write(|w| ..)` method takes [intr_state0::W](intr_state0::W) writer structure"]
impl crate::Writable for INTR_STATE0 {}
#[doc = "Interrupt Status"]
pub mod intr_state0;
#[doc = "Interrupt test register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_test0](intr_test0) module"]
pub type INTR_TEST0 = crate::Reg<u32, _INTR_TEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TEST0;
#[doc = "`write(|w| ..)` method takes [intr_test0::W](intr_test0::W) writer structure"]
impl crate::Writable for INTR_TEST0 {}
#[doc = "Interrupt test register"]
pub mod intr_test0;
