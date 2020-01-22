#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
    #[doc = "0x0c - Register write enable for all control registers."]
    pub regen: REGEN,
    _reserved4: [u8; 4usize],
    #[doc = "0x10 - Ping timeout cycle count."]
    pub ping_timeout_cyc: PING_TIMEOUT_CYC,
    _reserved5: [u8; 1usize],
    #[doc = "0x14 - Enable register for alerts."]
    pub alert_en: ALERT_EN,
    #[doc = "0x18 - Class assignment of alerts."]
    pub alert_class: ALERT_CLASS,
    #[doc = "0x1c - Alert Cause Register"]
    pub alert_cause: ALERT_CAUSE,
    #[doc = "0x20 - Enable register for the aggregated local alerts \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
    pub loc_alert_en: LOC_ALERT_EN,
    #[doc = "0x24 - Class assignment of local alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
    pub loc_alert_class: LOC_ALERT_CLASS,
    #[doc = "0x28 - Alert Cause Register for Local Alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
    pub loc_alert_cause: LOC_ALERT_CAUSE,
    #[doc = "0x2c - Escalation control register for alert Class A. Can not be modified if !!REGEN is false."]
    pub classa_ctrl: CLASSA_CTRL,
    #[doc = "0x30 - Clear enable for escalation protocol of Class A alerts."]
    pub classa_clren: CLASSA_CLREN,
    _reserved13: [u8; 4usize],
    #[doc = "0x34 - Clear for esclation protocol of Class A."]
    pub classa_clr: CLASSA_CLR,
    _reserved14: [u8; 4usize],
    #[doc = "0x38 - Current accumulation value for alert Class A. Software can clear this register with a write to !!CLASSA_CLR register unless !!CLASSA_CLREN is false."]
    pub classa_accum_cnt: CLASSA_ACCUM_CNT,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - Accumulation threshold value for alert Class A."]
    pub classa_accum_thresh: CLASSA_ACCUM_THRESH,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - Interrupt timeout in cycles."]
    pub classa_timeout_cyc: CLASSA_TIMEOUT_CYC,
    #[doc = "0x44 - Duration of escalation phase 0 for Class A."]
    pub classa_phase0_cyc: CLASSA_PHASE0_CYC,
    #[doc = "0x48 - Duration of escalation phase 1 for Class A."]
    pub classa_phase1_cyc: CLASSA_PHASE1_CYC,
    #[doc = "0x4c - Duration of escalation phase 2 for Class A."]
    pub classa_phase2_cyc: CLASSA_PHASE2_CYC,
    #[doc = "0x50 - Duration of escalation phase 3 for Class A."]
    pub classa_phase3_cyc: CLASSA_PHASE3_CYC,
    #[doc = "0x54 - Escalation counter in cycles for Class A."]
    pub classa_esc_cnt: CLASSA_ESC_CNT,
    #[doc = "0x58 - Current escalation state of Class A. See also !!CLASSA_ESC_CNT."]
    pub classa_state: CLASSA_STATE,
    _reserved23: [u8; 4usize],
    #[doc = "0x5c - Escalation control register for alert Class B. Can not be modified if !!REGEN is false."]
    pub classb_ctrl: CLASSB_CTRL,
    #[doc = "0x60 - Clear enable for escalation protocol of Class B alerts."]
    pub classb_clren: CLASSB_CLREN,
    _reserved25: [u8; 4usize],
    #[doc = "0x64 - Clear for esclation protocol of Class B."]
    pub classb_clr: CLASSB_CLR,
    _reserved26: [u8; 4usize],
    #[doc = "0x68 - Current accumulation value for alert Class B. Software can clear this register with a write to !!CLASSB_CLR register unless !!CLASSB_CLREN is false."]
    pub classb_accum_cnt: CLASSB_ACCUM_CNT,
    _reserved27: [u8; 2usize],
    #[doc = "0x6c - Accumulation threshold value for alert Class B."]
    pub classb_accum_thresh: CLASSB_ACCUM_THRESH,
    _reserved28: [u8; 2usize],
    #[doc = "0x70 - Interrupt timeout in cycles."]
    pub classb_timeout_cyc: CLASSB_TIMEOUT_CYC,
    #[doc = "0x74 - Duration of escalation phase 0 for Class B."]
    pub classb_phase0_cyc: CLASSB_PHASE0_CYC,
    #[doc = "0x78 - Duration of escalation phase 1 for Class B."]
    pub classb_phase1_cyc: CLASSB_PHASE1_CYC,
    #[doc = "0x7c - Duration of escalation phase 2 for Class B."]
    pub classb_phase2_cyc: CLASSB_PHASE2_CYC,
    #[doc = "0x80 - Duration of escalation phase 3 for Class B."]
    pub classb_phase3_cyc: CLASSB_PHASE3_CYC,
    #[doc = "0x84 - Escalation counter in cycles for Class B."]
    pub classb_esc_cnt: CLASSB_ESC_CNT,
    #[doc = "0x88 - Current escalation state of Class B. See also !!CLASSB_ESC_CNT."]
    pub classb_state: CLASSB_STATE,
    _reserved35: [u8; 4usize],
    #[doc = "0x8c - Escalation control register for alert Class C. Can not be modified if !!REGEN is false."]
    pub classc_ctrl: CLASSC_CTRL,
    #[doc = "0x90 - Clear enable for escalation protocol of Class C alerts."]
    pub classc_clren: CLASSC_CLREN,
    _reserved37: [u8; 4usize],
    #[doc = "0x94 - Clear for esclation protocol of Class C."]
    pub classc_clr: CLASSC_CLR,
    _reserved38: [u8; 4usize],
    #[doc = "0x98 - Current accumulation value for alert Class C. Software can clear this register with a write to !!CLASSC_CLR register unless !!CLASSC_CLREN is false."]
    pub classc_accum_cnt: CLASSC_ACCUM_CNT,
    _reserved39: [u8; 2usize],
    #[doc = "0x9c - Accumulation threshold value for alert Class C."]
    pub classc_accum_thresh: CLASSC_ACCUM_THRESH,
    _reserved40: [u8; 2usize],
    #[doc = "0xa0 - Interrupt timeout in cycles."]
    pub classc_timeout_cyc: CLASSC_TIMEOUT_CYC,
    #[doc = "0xa4 - Duration of escalation phase 0 for Class C."]
    pub classc_phase0_cyc: CLASSC_PHASE0_CYC,
    #[doc = "0xa8 - Duration of escalation phase 1 for Class C."]
    pub classc_phase1_cyc: CLASSC_PHASE1_CYC,
    #[doc = "0xac - Duration of escalation phase 2 for Class C."]
    pub classc_phase2_cyc: CLASSC_PHASE2_CYC,
    #[doc = "0xb0 - Duration of escalation phase 3 for Class C."]
    pub classc_phase3_cyc: CLASSC_PHASE3_CYC,
    #[doc = "0xb4 - Escalation counter in cycles for Class C."]
    pub classc_esc_cnt: CLASSC_ESC_CNT,
    #[doc = "0xb8 - Current escalation state of Class C. See also !!CLASSC_ESC_CNT."]
    pub classc_state: CLASSC_STATE,
    _reserved47: [u8; 4usize],
    #[doc = "0xbc - Escalation control register for alert Class D. Can not be modified if !!REGEN is false."]
    pub classd_ctrl: CLASSD_CTRL,
    #[doc = "0xc0 - Clear enable for escalation protocol of Class D alerts."]
    pub classd_clren: CLASSD_CLREN,
    _reserved49: [u8; 4usize],
    #[doc = "0xc4 - Clear for esclation protocol of Class D."]
    pub classd_clr: CLASSD_CLR,
    _reserved50: [u8; 4usize],
    #[doc = "0xc8 - Current accumulation value for alert Class D. Software can clear this register with a write to !!CLASSD_CLR register unless !!CLASSD_CLREN is false."]
    pub classd_accum_cnt: CLASSD_ACCUM_CNT,
    _reserved51: [u8; 2usize],
    #[doc = "0xcc - Accumulation threshold value for alert Class D."]
    pub classd_accum_thresh: CLASSD_ACCUM_THRESH,
    _reserved52: [u8; 2usize],
    #[doc = "0xd0 - Interrupt timeout in cycles."]
    pub classd_timeout_cyc: CLASSD_TIMEOUT_CYC,
    #[doc = "0xd4 - Duration of escalation phase 0 for Class D."]
    pub classd_phase0_cyc: CLASSD_PHASE0_CYC,
    #[doc = "0xd8 - Duration of escalation phase 1 for Class D."]
    pub classd_phase1_cyc: CLASSD_PHASE1_CYC,
    #[doc = "0xdc - Duration of escalation phase 2 for Class D."]
    pub classd_phase2_cyc: CLASSD_PHASE2_CYC,
    #[doc = "0xe0 - Duration of escalation phase 3 for Class D."]
    pub classd_phase3_cyc: CLASSD_PHASE3_CYC,
    #[doc = "0xe4 - Escalation counter in cycles for Class D."]
    pub classd_esc_cnt: CLASSD_ESC_CNT,
    #[doc = "0xe8 - Current escalation state of Class D. See also !!CLASSD_ESC_CNT."]
    pub classd_state: CLASSD_STATE,
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
#[doc = "Register write enable for all control registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regen](regen) module"]
pub type REGEN = crate::Reg<u8, _REGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGEN;
#[doc = "`read()` method returns [regen::R](regen::R) reader structure"]
impl crate::Readable for REGEN {}
#[doc = "`write(|w| ..)` method takes [regen::W](regen::W) writer structure"]
impl crate::Writable for REGEN {}
#[doc = "Register write enable for all control registers."]
pub mod regen;
#[doc = "Ping timeout cycle count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ping_timeout_cyc](ping_timeout_cyc) module"]
pub type PING_TIMEOUT_CYC = crate::Reg<u32, _PING_TIMEOUT_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PING_TIMEOUT_CYC;
#[doc = "`read()` method returns [ping_timeout_cyc::R](ping_timeout_cyc::R) reader structure"]
impl crate::Readable for PING_TIMEOUT_CYC {}
#[doc = "`write(|w| ..)` method takes [ping_timeout_cyc::W](ping_timeout_cyc::W) writer structure"]
impl crate::Writable for PING_TIMEOUT_CYC {}
#[doc = "Ping timeout cycle count."]
pub mod ping_timeout_cyc;
#[doc = "Enable register for alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alert_en](alert_en) module"]
pub type ALERT_EN = crate::Reg<u32, _ALERT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALERT_EN;
#[doc = "`read()` method returns [alert_en::R](alert_en::R) reader structure"]
impl crate::Readable for ALERT_EN {}
#[doc = "`write(|w| ..)` method takes [alert_en::W](alert_en::W) writer structure"]
impl crate::Writable for ALERT_EN {}
#[doc = "Enable register for alerts."]
pub mod alert_en;
#[doc = "Class assignment of alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alert_class](alert_class) module"]
pub type ALERT_CLASS = crate::Reg<u32, _ALERT_CLASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALERT_CLASS;
#[doc = "`read()` method returns [alert_class::R](alert_class::R) reader structure"]
impl crate::Readable for ALERT_CLASS {}
#[doc = "`write(|w| ..)` method takes [alert_class::W](alert_class::W) writer structure"]
impl crate::Writable for ALERT_CLASS {}
#[doc = "Class assignment of alerts."]
pub mod alert_class;
#[doc = "Alert Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alert_cause](alert_cause) module"]
pub type ALERT_CAUSE = crate::Reg<u32, _ALERT_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALERT_CAUSE;
#[doc = "`read()` method returns [alert_cause::R](alert_cause::R) reader structure"]
impl crate::Readable for ALERT_CAUSE {}
#[doc = "`write(|w| ..)` method takes [alert_cause::W](alert_cause::W) writer structure"]
impl crate::Writable for ALERT_CAUSE {}
#[doc = "Alert Cause Register"]
pub mod alert_cause;
#[doc = "Enable register for the aggregated local alerts \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loc_alert_en](loc_alert_en) module"]
pub type LOC_ALERT_EN = crate::Reg<u32, _LOC_ALERT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOC_ALERT_EN;
#[doc = "`read()` method returns [loc_alert_en::R](loc_alert_en::R) reader structure"]
impl crate::Readable for LOC_ALERT_EN {}
#[doc = "`write(|w| ..)` method takes [loc_alert_en::W](loc_alert_en::W) writer structure"]
impl crate::Writable for LOC_ALERT_EN {}
#[doc = "Enable register for the aggregated local alerts \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
pub mod loc_alert_en;
#[doc = "Class assignment of local alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loc_alert_class](loc_alert_class) module"]
pub type LOC_ALERT_CLASS = crate::Reg<u32, _LOC_ALERT_CLASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOC_ALERT_CLASS;
#[doc = "`read()` method returns [loc_alert_class::R](loc_alert_class::R) reader structure"]
impl crate::Readable for LOC_ALERT_CLASS {}
#[doc = "`write(|w| ..)` method takes [loc_alert_class::W](loc_alert_class::W) writer structure"]
impl crate::Writable for LOC_ALERT_CLASS {}
#[doc = "Class assignment of local alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
pub mod loc_alert_class;
#[doc = "Alert Cause Register for Local Alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loc_alert_cause](loc_alert_cause) module"]
pub type LOC_ALERT_CAUSE = crate::Reg<u32, _LOC_ALERT_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOC_ALERT_CAUSE;
#[doc = "`read()` method returns [loc_alert_cause::R](loc_alert_cause::R) reader structure"]
impl crate::Readable for LOC_ALERT_CAUSE {}
#[doc = "`write(|w| ..)` method takes [loc_alert_cause::W](loc_alert_cause::W) writer structure"]
impl crate::Writable for LOC_ALERT_CAUSE {}
#[doc = "Alert Cause Register for Local Alerts. \"alert pingfail\" (0), \"escalation pingfail\" (1), \"alert integfail\" (2) and \"escalation integfail\" (3)."]
pub mod loc_alert_cause;
#[doc = "Escalation control register for alert Class A. Can not be modified if !!REGEN is false.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_ctrl](classa_ctrl) module"]
pub type CLASSA_CTRL = crate::Reg<u32, _CLASSA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_CTRL;
#[doc = "`read()` method returns [classa_ctrl::R](classa_ctrl::R) reader structure"]
impl crate::Readable for CLASSA_CTRL {}
#[doc = "`write(|w| ..)` method takes [classa_ctrl::W](classa_ctrl::W) writer structure"]
impl crate::Writable for CLASSA_CTRL {}
#[doc = "Escalation control register for alert Class A. Can not be modified if !!REGEN is false."]
pub mod classa_ctrl;
#[doc = "Clear enable for escalation protocol of Class A alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_clren](classa_clren) module"]
pub type CLASSA_CLREN = crate::Reg<u8, _CLASSA_CLREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_CLREN;
#[doc = "`read()` method returns [classa_clren::R](classa_clren::R) reader structure"]
impl crate::Readable for CLASSA_CLREN {}
#[doc = "`write(|w| ..)` method takes [classa_clren::W](classa_clren::W) writer structure"]
impl crate::Writable for CLASSA_CLREN {}
#[doc = "Clear enable for escalation protocol of Class A alerts."]
pub mod classa_clren;
#[doc = "Clear for esclation protocol of Class A.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_clr](classa_clr) module"]
pub type CLASSA_CLR = crate::Reg<u8, _CLASSA_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_CLR;
#[doc = "`write(|w| ..)` method takes [classa_clr::W](classa_clr::W) writer structure"]
impl crate::Writable for CLASSA_CLR {}
#[doc = "Clear for esclation protocol of Class A."]
pub mod classa_clr;
#[doc = "Current accumulation value for alert Class A. Software can clear this register with a write to !!CLASSA_CLR register unless !!CLASSA_CLREN is false.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_accum_cnt](classa_accum_cnt) module"]
pub type CLASSA_ACCUM_CNT = crate::Reg<u16, _CLASSA_ACCUM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_ACCUM_CNT;
#[doc = "`read()` method returns [classa_accum_cnt::R](classa_accum_cnt::R) reader structure"]
impl crate::Readable for CLASSA_ACCUM_CNT {}
#[doc = "Current accumulation value for alert Class A. Software can clear this register with a write to !!CLASSA_CLR register unless !!CLASSA_CLREN is false."]
pub mod classa_accum_cnt;
#[doc = "Accumulation threshold value for alert Class A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_accum_thresh](classa_accum_thresh) module"]
pub type CLASSA_ACCUM_THRESH = crate::Reg<u16, _CLASSA_ACCUM_THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_ACCUM_THRESH;
#[doc = "`read()` method returns [classa_accum_thresh::R](classa_accum_thresh::R) reader structure"]
impl crate::Readable for CLASSA_ACCUM_THRESH {}
#[doc = "`write(|w| ..)` method takes [classa_accum_thresh::W](classa_accum_thresh::W) writer structure"]
impl crate::Writable for CLASSA_ACCUM_THRESH {}
#[doc = "Accumulation threshold value for alert Class A."]
pub mod classa_accum_thresh;
#[doc = "Interrupt timeout in cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_timeout_cyc](classa_timeout_cyc) module"]
pub type CLASSA_TIMEOUT_CYC = crate::Reg<u32, _CLASSA_TIMEOUT_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_TIMEOUT_CYC;
#[doc = "`read()` method returns [classa_timeout_cyc::R](classa_timeout_cyc::R) reader structure"]
impl crate::Readable for CLASSA_TIMEOUT_CYC {}
#[doc = "`write(|w| ..)` method takes [classa_timeout_cyc::W](classa_timeout_cyc::W) writer structure"]
impl crate::Writable for CLASSA_TIMEOUT_CYC {}
#[doc = "Interrupt timeout in cycles."]
pub mod classa_timeout_cyc;
#[doc = "Duration of escalation phase 0 for Class A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_phase0_cyc](classa_phase0_cyc) module"]
pub type CLASSA_PHASE0_CYC = crate::Reg<u32, _CLASSA_PHASE0_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_PHASE0_CYC;
#[doc = "`read()` method returns [classa_phase0_cyc::R](classa_phase0_cyc::R) reader structure"]
impl crate::Readable for CLASSA_PHASE0_CYC {}
#[doc = "`write(|w| ..)` method takes [classa_phase0_cyc::W](classa_phase0_cyc::W) writer structure"]
impl crate::Writable for CLASSA_PHASE0_CYC {}
#[doc = "Duration of escalation phase 0 for Class A."]
pub mod classa_phase0_cyc;
#[doc = "Duration of escalation phase 1 for Class A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_phase1_cyc](classa_phase1_cyc) module"]
pub type CLASSA_PHASE1_CYC = crate::Reg<u32, _CLASSA_PHASE1_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_PHASE1_CYC;
#[doc = "`read()` method returns [classa_phase1_cyc::R](classa_phase1_cyc::R) reader structure"]
impl crate::Readable for CLASSA_PHASE1_CYC {}
#[doc = "`write(|w| ..)` method takes [classa_phase1_cyc::W](classa_phase1_cyc::W) writer structure"]
impl crate::Writable for CLASSA_PHASE1_CYC {}
#[doc = "Duration of escalation phase 1 for Class A."]
pub mod classa_phase1_cyc;
#[doc = "Duration of escalation phase 2 for Class A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_phase2_cyc](classa_phase2_cyc) module"]
pub type CLASSA_PHASE2_CYC = crate::Reg<u32, _CLASSA_PHASE2_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_PHASE2_CYC;
#[doc = "`read()` method returns [classa_phase2_cyc::R](classa_phase2_cyc::R) reader structure"]
impl crate::Readable for CLASSA_PHASE2_CYC {}
#[doc = "`write(|w| ..)` method takes [classa_phase2_cyc::W](classa_phase2_cyc::W) writer structure"]
impl crate::Writable for CLASSA_PHASE2_CYC {}
#[doc = "Duration of escalation phase 2 for Class A."]
pub mod classa_phase2_cyc;
#[doc = "Duration of escalation phase 3 for Class A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_phase3_cyc](classa_phase3_cyc) module"]
pub type CLASSA_PHASE3_CYC = crate::Reg<u32, _CLASSA_PHASE3_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_PHASE3_CYC;
#[doc = "`read()` method returns [classa_phase3_cyc::R](classa_phase3_cyc::R) reader structure"]
impl crate::Readable for CLASSA_PHASE3_CYC {}
#[doc = "`write(|w| ..)` method takes [classa_phase3_cyc::W](classa_phase3_cyc::W) writer structure"]
impl crate::Writable for CLASSA_PHASE3_CYC {}
#[doc = "Duration of escalation phase 3 for Class A."]
pub mod classa_phase3_cyc;
#[doc = "Escalation counter in cycles for Class A.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_esc_cnt](classa_esc_cnt) module"]
pub type CLASSA_ESC_CNT = crate::Reg<u32, _CLASSA_ESC_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_ESC_CNT;
#[doc = "`read()` method returns [classa_esc_cnt::R](classa_esc_cnt::R) reader structure"]
impl crate::Readable for CLASSA_ESC_CNT {}
#[doc = "Escalation counter in cycles for Class A."]
pub mod classa_esc_cnt;
#[doc = "Current escalation state of Class A. See also !!CLASSA_ESC_CNT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classa_state](classa_state) module"]
pub type CLASSA_STATE = crate::Reg<u8, _CLASSA_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSA_STATE;
#[doc = "`read()` method returns [classa_state::R](classa_state::R) reader structure"]
impl crate::Readable for CLASSA_STATE {}
#[doc = "Current escalation state of Class A. See also !!CLASSA_ESC_CNT."]
pub mod classa_state;
#[doc = "Escalation control register for alert Class B. Can not be modified if !!REGEN is false.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_ctrl](classb_ctrl) module"]
pub type CLASSB_CTRL = crate::Reg<u32, _CLASSB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_CTRL;
#[doc = "`read()` method returns [classb_ctrl::R](classb_ctrl::R) reader structure"]
impl crate::Readable for CLASSB_CTRL {}
#[doc = "`write(|w| ..)` method takes [classb_ctrl::W](classb_ctrl::W) writer structure"]
impl crate::Writable for CLASSB_CTRL {}
#[doc = "Escalation control register for alert Class B. Can not be modified if !!REGEN is false."]
pub mod classb_ctrl;
#[doc = "Clear enable for escalation protocol of Class B alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_clren](classb_clren) module"]
pub type CLASSB_CLREN = crate::Reg<u8, _CLASSB_CLREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_CLREN;
#[doc = "`read()` method returns [classb_clren::R](classb_clren::R) reader structure"]
impl crate::Readable for CLASSB_CLREN {}
#[doc = "`write(|w| ..)` method takes [classb_clren::W](classb_clren::W) writer structure"]
impl crate::Writable for CLASSB_CLREN {}
#[doc = "Clear enable for escalation protocol of Class B alerts."]
pub mod classb_clren;
#[doc = "Clear for esclation protocol of Class B.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_clr](classb_clr) module"]
pub type CLASSB_CLR = crate::Reg<u8, _CLASSB_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_CLR;
#[doc = "`write(|w| ..)` method takes [classb_clr::W](classb_clr::W) writer structure"]
impl crate::Writable for CLASSB_CLR {}
#[doc = "Clear for esclation protocol of Class B."]
pub mod classb_clr;
#[doc = "Current accumulation value for alert Class B. Software can clear this register with a write to !!CLASSB_CLR register unless !!CLASSB_CLREN is false.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_accum_cnt](classb_accum_cnt) module"]
pub type CLASSB_ACCUM_CNT = crate::Reg<u16, _CLASSB_ACCUM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_ACCUM_CNT;
#[doc = "`read()` method returns [classb_accum_cnt::R](classb_accum_cnt::R) reader structure"]
impl crate::Readable for CLASSB_ACCUM_CNT {}
#[doc = "Current accumulation value for alert Class B. Software can clear this register with a write to !!CLASSB_CLR register unless !!CLASSB_CLREN is false."]
pub mod classb_accum_cnt;
#[doc = "Accumulation threshold value for alert Class B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_accum_thresh](classb_accum_thresh) module"]
pub type CLASSB_ACCUM_THRESH = crate::Reg<u16, _CLASSB_ACCUM_THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_ACCUM_THRESH;
#[doc = "`read()` method returns [classb_accum_thresh::R](classb_accum_thresh::R) reader structure"]
impl crate::Readable for CLASSB_ACCUM_THRESH {}
#[doc = "`write(|w| ..)` method takes [classb_accum_thresh::W](classb_accum_thresh::W) writer structure"]
impl crate::Writable for CLASSB_ACCUM_THRESH {}
#[doc = "Accumulation threshold value for alert Class B."]
pub mod classb_accum_thresh;
#[doc = "Interrupt timeout in cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_timeout_cyc](classb_timeout_cyc) module"]
pub type CLASSB_TIMEOUT_CYC = crate::Reg<u32, _CLASSB_TIMEOUT_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_TIMEOUT_CYC;
#[doc = "`read()` method returns [classb_timeout_cyc::R](classb_timeout_cyc::R) reader structure"]
impl crate::Readable for CLASSB_TIMEOUT_CYC {}
#[doc = "`write(|w| ..)` method takes [classb_timeout_cyc::W](classb_timeout_cyc::W) writer structure"]
impl crate::Writable for CLASSB_TIMEOUT_CYC {}
#[doc = "Interrupt timeout in cycles."]
pub mod classb_timeout_cyc;
#[doc = "Duration of escalation phase 0 for Class B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_phase0_cyc](classb_phase0_cyc) module"]
pub type CLASSB_PHASE0_CYC = crate::Reg<u32, _CLASSB_PHASE0_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_PHASE0_CYC;
#[doc = "`read()` method returns [classb_phase0_cyc::R](classb_phase0_cyc::R) reader structure"]
impl crate::Readable for CLASSB_PHASE0_CYC {}
#[doc = "`write(|w| ..)` method takes [classb_phase0_cyc::W](classb_phase0_cyc::W) writer structure"]
impl crate::Writable for CLASSB_PHASE0_CYC {}
#[doc = "Duration of escalation phase 0 for Class B."]
pub mod classb_phase0_cyc;
#[doc = "Duration of escalation phase 1 for Class B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_phase1_cyc](classb_phase1_cyc) module"]
pub type CLASSB_PHASE1_CYC = crate::Reg<u32, _CLASSB_PHASE1_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_PHASE1_CYC;
#[doc = "`read()` method returns [classb_phase1_cyc::R](classb_phase1_cyc::R) reader structure"]
impl crate::Readable for CLASSB_PHASE1_CYC {}
#[doc = "`write(|w| ..)` method takes [classb_phase1_cyc::W](classb_phase1_cyc::W) writer structure"]
impl crate::Writable for CLASSB_PHASE1_CYC {}
#[doc = "Duration of escalation phase 1 for Class B."]
pub mod classb_phase1_cyc;
#[doc = "Duration of escalation phase 2 for Class B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_phase2_cyc](classb_phase2_cyc) module"]
pub type CLASSB_PHASE2_CYC = crate::Reg<u32, _CLASSB_PHASE2_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_PHASE2_CYC;
#[doc = "`read()` method returns [classb_phase2_cyc::R](classb_phase2_cyc::R) reader structure"]
impl crate::Readable for CLASSB_PHASE2_CYC {}
#[doc = "`write(|w| ..)` method takes [classb_phase2_cyc::W](classb_phase2_cyc::W) writer structure"]
impl crate::Writable for CLASSB_PHASE2_CYC {}
#[doc = "Duration of escalation phase 2 for Class B."]
pub mod classb_phase2_cyc;
#[doc = "Duration of escalation phase 3 for Class B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_phase3_cyc](classb_phase3_cyc) module"]
pub type CLASSB_PHASE3_CYC = crate::Reg<u32, _CLASSB_PHASE3_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_PHASE3_CYC;
#[doc = "`read()` method returns [classb_phase3_cyc::R](classb_phase3_cyc::R) reader structure"]
impl crate::Readable for CLASSB_PHASE3_CYC {}
#[doc = "`write(|w| ..)` method takes [classb_phase3_cyc::W](classb_phase3_cyc::W) writer structure"]
impl crate::Writable for CLASSB_PHASE3_CYC {}
#[doc = "Duration of escalation phase 3 for Class B."]
pub mod classb_phase3_cyc;
#[doc = "Escalation counter in cycles for Class B.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_esc_cnt](classb_esc_cnt) module"]
pub type CLASSB_ESC_CNT = crate::Reg<u32, _CLASSB_ESC_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_ESC_CNT;
#[doc = "`read()` method returns [classb_esc_cnt::R](classb_esc_cnt::R) reader structure"]
impl crate::Readable for CLASSB_ESC_CNT {}
#[doc = "Escalation counter in cycles for Class B."]
pub mod classb_esc_cnt;
#[doc = "Current escalation state of Class B. See also !!CLASSB_ESC_CNT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classb_state](classb_state) module"]
pub type CLASSB_STATE = crate::Reg<u8, _CLASSB_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSB_STATE;
#[doc = "`read()` method returns [classb_state::R](classb_state::R) reader structure"]
impl crate::Readable for CLASSB_STATE {}
#[doc = "Current escalation state of Class B. See also !!CLASSB_ESC_CNT."]
pub mod classb_state;
#[doc = "Escalation control register for alert Class C. Can not be modified if !!REGEN is false.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_ctrl](classc_ctrl) module"]
pub type CLASSC_CTRL = crate::Reg<u32, _CLASSC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_CTRL;
#[doc = "`read()` method returns [classc_ctrl::R](classc_ctrl::R) reader structure"]
impl crate::Readable for CLASSC_CTRL {}
#[doc = "`write(|w| ..)` method takes [classc_ctrl::W](classc_ctrl::W) writer structure"]
impl crate::Writable for CLASSC_CTRL {}
#[doc = "Escalation control register for alert Class C. Can not be modified if !!REGEN is false."]
pub mod classc_ctrl;
#[doc = "Clear enable for escalation protocol of Class C alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_clren](classc_clren) module"]
pub type CLASSC_CLREN = crate::Reg<u8, _CLASSC_CLREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_CLREN;
#[doc = "`read()` method returns [classc_clren::R](classc_clren::R) reader structure"]
impl crate::Readable for CLASSC_CLREN {}
#[doc = "`write(|w| ..)` method takes [classc_clren::W](classc_clren::W) writer structure"]
impl crate::Writable for CLASSC_CLREN {}
#[doc = "Clear enable for escalation protocol of Class C alerts."]
pub mod classc_clren;
#[doc = "Clear for esclation protocol of Class C.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_clr](classc_clr) module"]
pub type CLASSC_CLR = crate::Reg<u8, _CLASSC_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_CLR;
#[doc = "`write(|w| ..)` method takes [classc_clr::W](classc_clr::W) writer structure"]
impl crate::Writable for CLASSC_CLR {}
#[doc = "Clear for esclation protocol of Class C."]
pub mod classc_clr;
#[doc = "Current accumulation value for alert Class C. Software can clear this register with a write to !!CLASSC_CLR register unless !!CLASSC_CLREN is false.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_accum_cnt](classc_accum_cnt) module"]
pub type CLASSC_ACCUM_CNT = crate::Reg<u16, _CLASSC_ACCUM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_ACCUM_CNT;
#[doc = "`read()` method returns [classc_accum_cnt::R](classc_accum_cnt::R) reader structure"]
impl crate::Readable for CLASSC_ACCUM_CNT {}
#[doc = "Current accumulation value for alert Class C. Software can clear this register with a write to !!CLASSC_CLR register unless !!CLASSC_CLREN is false."]
pub mod classc_accum_cnt;
#[doc = "Accumulation threshold value for alert Class C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_accum_thresh](classc_accum_thresh) module"]
pub type CLASSC_ACCUM_THRESH = crate::Reg<u16, _CLASSC_ACCUM_THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_ACCUM_THRESH;
#[doc = "`read()` method returns [classc_accum_thresh::R](classc_accum_thresh::R) reader structure"]
impl crate::Readable for CLASSC_ACCUM_THRESH {}
#[doc = "`write(|w| ..)` method takes [classc_accum_thresh::W](classc_accum_thresh::W) writer structure"]
impl crate::Writable for CLASSC_ACCUM_THRESH {}
#[doc = "Accumulation threshold value for alert Class C."]
pub mod classc_accum_thresh;
#[doc = "Interrupt timeout in cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_timeout_cyc](classc_timeout_cyc) module"]
pub type CLASSC_TIMEOUT_CYC = crate::Reg<u32, _CLASSC_TIMEOUT_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_TIMEOUT_CYC;
#[doc = "`read()` method returns [classc_timeout_cyc::R](classc_timeout_cyc::R) reader structure"]
impl crate::Readable for CLASSC_TIMEOUT_CYC {}
#[doc = "`write(|w| ..)` method takes [classc_timeout_cyc::W](classc_timeout_cyc::W) writer structure"]
impl crate::Writable for CLASSC_TIMEOUT_CYC {}
#[doc = "Interrupt timeout in cycles."]
pub mod classc_timeout_cyc;
#[doc = "Duration of escalation phase 0 for Class C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_phase0_cyc](classc_phase0_cyc) module"]
pub type CLASSC_PHASE0_CYC = crate::Reg<u32, _CLASSC_PHASE0_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_PHASE0_CYC;
#[doc = "`read()` method returns [classc_phase0_cyc::R](classc_phase0_cyc::R) reader structure"]
impl crate::Readable for CLASSC_PHASE0_CYC {}
#[doc = "`write(|w| ..)` method takes [classc_phase0_cyc::W](classc_phase0_cyc::W) writer structure"]
impl crate::Writable for CLASSC_PHASE0_CYC {}
#[doc = "Duration of escalation phase 0 for Class C."]
pub mod classc_phase0_cyc;
#[doc = "Duration of escalation phase 1 for Class C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_phase1_cyc](classc_phase1_cyc) module"]
pub type CLASSC_PHASE1_CYC = crate::Reg<u32, _CLASSC_PHASE1_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_PHASE1_CYC;
#[doc = "`read()` method returns [classc_phase1_cyc::R](classc_phase1_cyc::R) reader structure"]
impl crate::Readable for CLASSC_PHASE1_CYC {}
#[doc = "`write(|w| ..)` method takes [classc_phase1_cyc::W](classc_phase1_cyc::W) writer structure"]
impl crate::Writable for CLASSC_PHASE1_CYC {}
#[doc = "Duration of escalation phase 1 for Class C."]
pub mod classc_phase1_cyc;
#[doc = "Duration of escalation phase 2 for Class C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_phase2_cyc](classc_phase2_cyc) module"]
pub type CLASSC_PHASE2_CYC = crate::Reg<u32, _CLASSC_PHASE2_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_PHASE2_CYC;
#[doc = "`read()` method returns [classc_phase2_cyc::R](classc_phase2_cyc::R) reader structure"]
impl crate::Readable for CLASSC_PHASE2_CYC {}
#[doc = "`write(|w| ..)` method takes [classc_phase2_cyc::W](classc_phase2_cyc::W) writer structure"]
impl crate::Writable for CLASSC_PHASE2_CYC {}
#[doc = "Duration of escalation phase 2 for Class C."]
pub mod classc_phase2_cyc;
#[doc = "Duration of escalation phase 3 for Class C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_phase3_cyc](classc_phase3_cyc) module"]
pub type CLASSC_PHASE3_CYC = crate::Reg<u32, _CLASSC_PHASE3_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_PHASE3_CYC;
#[doc = "`read()` method returns [classc_phase3_cyc::R](classc_phase3_cyc::R) reader structure"]
impl crate::Readable for CLASSC_PHASE3_CYC {}
#[doc = "`write(|w| ..)` method takes [classc_phase3_cyc::W](classc_phase3_cyc::W) writer structure"]
impl crate::Writable for CLASSC_PHASE3_CYC {}
#[doc = "Duration of escalation phase 3 for Class C."]
pub mod classc_phase3_cyc;
#[doc = "Escalation counter in cycles for Class C.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_esc_cnt](classc_esc_cnt) module"]
pub type CLASSC_ESC_CNT = crate::Reg<u32, _CLASSC_ESC_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_ESC_CNT;
#[doc = "`read()` method returns [classc_esc_cnt::R](classc_esc_cnt::R) reader structure"]
impl crate::Readable for CLASSC_ESC_CNT {}
#[doc = "Escalation counter in cycles for Class C."]
pub mod classc_esc_cnt;
#[doc = "Current escalation state of Class C. See also !!CLASSC_ESC_CNT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classc_state](classc_state) module"]
pub type CLASSC_STATE = crate::Reg<u8, _CLASSC_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSC_STATE;
#[doc = "`read()` method returns [classc_state::R](classc_state::R) reader structure"]
impl crate::Readable for CLASSC_STATE {}
#[doc = "Current escalation state of Class C. See also !!CLASSC_ESC_CNT."]
pub mod classc_state;
#[doc = "Escalation control register for alert Class D. Can not be modified if !!REGEN is false.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_ctrl](classd_ctrl) module"]
pub type CLASSD_CTRL = crate::Reg<u32, _CLASSD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_CTRL;
#[doc = "`read()` method returns [classd_ctrl::R](classd_ctrl::R) reader structure"]
impl crate::Readable for CLASSD_CTRL {}
#[doc = "`write(|w| ..)` method takes [classd_ctrl::W](classd_ctrl::W) writer structure"]
impl crate::Writable for CLASSD_CTRL {}
#[doc = "Escalation control register for alert Class D. Can not be modified if !!REGEN is false."]
pub mod classd_ctrl;
#[doc = "Clear enable for escalation protocol of Class D alerts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_clren](classd_clren) module"]
pub type CLASSD_CLREN = crate::Reg<u8, _CLASSD_CLREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_CLREN;
#[doc = "`read()` method returns [classd_clren::R](classd_clren::R) reader structure"]
impl crate::Readable for CLASSD_CLREN {}
#[doc = "`write(|w| ..)` method takes [classd_clren::W](classd_clren::W) writer structure"]
impl crate::Writable for CLASSD_CLREN {}
#[doc = "Clear enable for escalation protocol of Class D alerts."]
pub mod classd_clren;
#[doc = "Clear for esclation protocol of Class D.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_clr](classd_clr) module"]
pub type CLASSD_CLR = crate::Reg<u8, _CLASSD_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_CLR;
#[doc = "`write(|w| ..)` method takes [classd_clr::W](classd_clr::W) writer structure"]
impl crate::Writable for CLASSD_CLR {}
#[doc = "Clear for esclation protocol of Class D."]
pub mod classd_clr;
#[doc = "Current accumulation value for alert Class D. Software can clear this register with a write to !!CLASSD_CLR register unless !!CLASSD_CLREN is false.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_accum_cnt](classd_accum_cnt) module"]
pub type CLASSD_ACCUM_CNT = crate::Reg<u16, _CLASSD_ACCUM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_ACCUM_CNT;
#[doc = "`read()` method returns [classd_accum_cnt::R](classd_accum_cnt::R) reader structure"]
impl crate::Readable for CLASSD_ACCUM_CNT {}
#[doc = "Current accumulation value for alert Class D. Software can clear this register with a write to !!CLASSD_CLR register unless !!CLASSD_CLREN is false."]
pub mod classd_accum_cnt;
#[doc = "Accumulation threshold value for alert Class D.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_accum_thresh](classd_accum_thresh) module"]
pub type CLASSD_ACCUM_THRESH = crate::Reg<u16, _CLASSD_ACCUM_THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_ACCUM_THRESH;
#[doc = "`read()` method returns [classd_accum_thresh::R](classd_accum_thresh::R) reader structure"]
impl crate::Readable for CLASSD_ACCUM_THRESH {}
#[doc = "`write(|w| ..)` method takes [classd_accum_thresh::W](classd_accum_thresh::W) writer structure"]
impl crate::Writable for CLASSD_ACCUM_THRESH {}
#[doc = "Accumulation threshold value for alert Class D."]
pub mod classd_accum_thresh;
#[doc = "Interrupt timeout in cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_timeout_cyc](classd_timeout_cyc) module"]
pub type CLASSD_TIMEOUT_CYC = crate::Reg<u32, _CLASSD_TIMEOUT_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_TIMEOUT_CYC;
#[doc = "`read()` method returns [classd_timeout_cyc::R](classd_timeout_cyc::R) reader structure"]
impl crate::Readable for CLASSD_TIMEOUT_CYC {}
#[doc = "`write(|w| ..)` method takes [classd_timeout_cyc::W](classd_timeout_cyc::W) writer structure"]
impl crate::Writable for CLASSD_TIMEOUT_CYC {}
#[doc = "Interrupt timeout in cycles."]
pub mod classd_timeout_cyc;
#[doc = "Duration of escalation phase 0 for Class D.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_phase0_cyc](classd_phase0_cyc) module"]
pub type CLASSD_PHASE0_CYC = crate::Reg<u32, _CLASSD_PHASE0_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_PHASE0_CYC;
#[doc = "`read()` method returns [classd_phase0_cyc::R](classd_phase0_cyc::R) reader structure"]
impl crate::Readable for CLASSD_PHASE0_CYC {}
#[doc = "`write(|w| ..)` method takes [classd_phase0_cyc::W](classd_phase0_cyc::W) writer structure"]
impl crate::Writable for CLASSD_PHASE0_CYC {}
#[doc = "Duration of escalation phase 0 for Class D."]
pub mod classd_phase0_cyc;
#[doc = "Duration of escalation phase 1 for Class D.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_phase1_cyc](classd_phase1_cyc) module"]
pub type CLASSD_PHASE1_CYC = crate::Reg<u32, _CLASSD_PHASE1_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_PHASE1_CYC;
#[doc = "`read()` method returns [classd_phase1_cyc::R](classd_phase1_cyc::R) reader structure"]
impl crate::Readable for CLASSD_PHASE1_CYC {}
#[doc = "`write(|w| ..)` method takes [classd_phase1_cyc::W](classd_phase1_cyc::W) writer structure"]
impl crate::Writable for CLASSD_PHASE1_CYC {}
#[doc = "Duration of escalation phase 1 for Class D."]
pub mod classd_phase1_cyc;
#[doc = "Duration of escalation phase 2 for Class D.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_phase2_cyc](classd_phase2_cyc) module"]
pub type CLASSD_PHASE2_CYC = crate::Reg<u32, _CLASSD_PHASE2_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_PHASE2_CYC;
#[doc = "`read()` method returns [classd_phase2_cyc::R](classd_phase2_cyc::R) reader structure"]
impl crate::Readable for CLASSD_PHASE2_CYC {}
#[doc = "`write(|w| ..)` method takes [classd_phase2_cyc::W](classd_phase2_cyc::W) writer structure"]
impl crate::Writable for CLASSD_PHASE2_CYC {}
#[doc = "Duration of escalation phase 2 for Class D."]
pub mod classd_phase2_cyc;
#[doc = "Duration of escalation phase 3 for Class D.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_phase3_cyc](classd_phase3_cyc) module"]
pub type CLASSD_PHASE3_CYC = crate::Reg<u32, _CLASSD_PHASE3_CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_PHASE3_CYC;
#[doc = "`read()` method returns [classd_phase3_cyc::R](classd_phase3_cyc::R) reader structure"]
impl crate::Readable for CLASSD_PHASE3_CYC {}
#[doc = "`write(|w| ..)` method takes [classd_phase3_cyc::W](classd_phase3_cyc::W) writer structure"]
impl crate::Writable for CLASSD_PHASE3_CYC {}
#[doc = "Duration of escalation phase 3 for Class D."]
pub mod classd_phase3_cyc;
#[doc = "Escalation counter in cycles for Class D.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_esc_cnt](classd_esc_cnt) module"]
pub type CLASSD_ESC_CNT = crate::Reg<u32, _CLASSD_ESC_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_ESC_CNT;
#[doc = "`read()` method returns [classd_esc_cnt::R](classd_esc_cnt::R) reader structure"]
impl crate::Readable for CLASSD_ESC_CNT {}
#[doc = "Escalation counter in cycles for Class D."]
pub mod classd_esc_cnt;
#[doc = "Current escalation state of Class D. See also !!CLASSD_ESC_CNT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [classd_state](classd_state) module"]
pub type CLASSD_STATE = crate::Reg<u8, _CLASSD_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLASSD_STATE;
#[doc = "`read()` method returns [classd_state::R](classd_state::R) reader structure"]
impl crate::Readable for CLASSD_STATE {}
#[doc = "Current escalation state of Class D. See also !!CLASSD_ESC_CNT."]
pub mod classd_state;
