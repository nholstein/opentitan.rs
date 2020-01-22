#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
    #[doc = "0x0c - Control register"]
    pub control: CONTROL,
    #[doc = "0x10 - Address for flash operation"]
    pub addr: ADDR,
    #[doc = "0x14 - Memory region registers configuration enable."]
    pub region_cfg_regwen: REGION_CFG_REGWEN,
    #[doc = "0x18 - Memory protection configuration"]
    pub mp_region_cfg: MP_REGION_CFG,
    #[doc = "0x38 - Default region permissions"]
    pub default_region: DEFAULT_REGION,
    #[doc = "0x3c - Bank configuration registers configuration enable."]
    pub bank_cfg_regwen: BANK_CFG_REGWEN,
    #[doc = "0x40 - Memory protect bank configuration"]
    pub mp_bank_cfg: MP_BANK_CFG,
    #[doc = "0x44 - Flash Operation Status"]
    pub op_status: OP_STATUS,
    #[doc = "0x48 - Flash Controller Status"]
    pub status: STATUS,
    #[doc = "0x4c - Flash Controller Scratch"]
    pub scratch: SCRATCH,
    #[doc = "0x50 - Programmable depth where fifos should generate interrupts"]
    pub fifo_lvl: FIFO_LVL,
    #[doc = "0x54 - Flash program fifo. The fifo is 16 entries of 4B flash words"]
    pub prog_fifo: [PROG_FIFO; 1],
    #[doc = "0x58 - Flash read fifo. The fifo is 16 entries of 4B flash words"]
    pub rd_fifo: [RD_FIFO; 1],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MP_REGION_CFG {
    #[doc = "0x00 - Memory protection configuration"]
    pub mp_region_cfg0: self::mp_region_cfg::MP_REGION_CFG0,
    #[doc = "0x04 - Memory protection configuration"]
    pub mp_region_cfg1: self::mp_region_cfg::MP_REGION_CFG1,
    #[doc = "0x08 - Memory protection configuration"]
    pub mp_region_cfg2: self::mp_region_cfg::MP_REGION_CFG2,
    #[doc = "0x0c - Memory protection configuration"]
    pub mp_region_cfg3: self::mp_region_cfg::MP_REGION_CFG3,
    #[doc = "0x10 - Memory protection configuration"]
    pub mp_region_cfg4: self::mp_region_cfg::MP_REGION_CFG4,
    #[doc = "0x14 - Memory protection configuration"]
    pub mp_region_cfg5: self::mp_region_cfg::MP_REGION_CFG5,
    #[doc = "0x18 - Memory protection configuration"]
    pub mp_region_cfg6: self::mp_region_cfg::MP_REGION_CFG6,
    #[doc = "0x1c - Memory protection configuration"]
    pub mp_region_cfg7: self::mp_region_cfg::MP_REGION_CFG7,
}
#[doc = r"Register block"]
#[doc = "Memory protection configuration"]
pub mod mp_region_cfg;
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
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Control register"]
pub mod control;
#[doc = "Address for flash operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address for flash operation"]
pub mod addr;
#[doc = "Memory region registers configuration enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_cfg_regwen](region_cfg_regwen) module"]
pub type REGION_CFG_REGWEN = crate::Reg<u32, _REGION_CFG_REGWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_CFG_REGWEN;
#[doc = "`read()` method returns [region_cfg_regwen::R](region_cfg_regwen::R) reader structure"]
impl crate::Readable for REGION_CFG_REGWEN {}
#[doc = "`write(|w| ..)` method takes [region_cfg_regwen::W](region_cfg_regwen::W) writer structure"]
impl crate::Writable for REGION_CFG_REGWEN {}
#[doc = "Memory region registers configuration enable."]
pub mod region_cfg_regwen;
#[doc = "Default region permissions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [default_region](default_region) module"]
pub type DEFAULT_REGION = crate::Reg<u32, _DEFAULT_REGION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEFAULT_REGION;
#[doc = "`read()` method returns [default_region::R](default_region::R) reader structure"]
impl crate::Readable for DEFAULT_REGION {}
#[doc = "`write(|w| ..)` method takes [default_region::W](default_region::W) writer structure"]
impl crate::Writable for DEFAULT_REGION {}
#[doc = "Default region permissions"]
pub mod default_region;
#[doc = "Bank configuration registers configuration enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank_cfg_regwen](bank_cfg_regwen) module"]
pub type BANK_CFG_REGWEN = crate::Reg<u32, _BANK_CFG_REGWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BANK_CFG_REGWEN;
#[doc = "`read()` method returns [bank_cfg_regwen::R](bank_cfg_regwen::R) reader structure"]
impl crate::Readable for BANK_CFG_REGWEN {}
#[doc = "`write(|w| ..)` method takes [bank_cfg_regwen::W](bank_cfg_regwen::W) writer structure"]
impl crate::Writable for BANK_CFG_REGWEN {}
#[doc = "Bank configuration registers configuration enable."]
pub mod bank_cfg_regwen;
#[doc = "Memory protect bank configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mp_bank_cfg](mp_bank_cfg) module"]
pub type MP_BANK_CFG = crate::Reg<u32, _MP_BANK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MP_BANK_CFG;
#[doc = "`read()` method returns [mp_bank_cfg::R](mp_bank_cfg::R) reader structure"]
impl crate::Readable for MP_BANK_CFG {}
#[doc = "`write(|w| ..)` method takes [mp_bank_cfg::W](mp_bank_cfg::W) writer structure"]
impl crate::Writable for MP_BANK_CFG {}
#[doc = "Memory protect bank configuration"]
pub mod mp_bank_cfg;
#[doc = "Flash Operation Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op_status](op_status) module"]
pub type OP_STATUS = crate::Reg<u32, _OP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP_STATUS;
#[doc = "`read()` method returns [op_status::R](op_status::R) reader structure"]
impl crate::Readable for OP_STATUS {}
#[doc = "`write(|w| ..)` method takes [op_status::W](op_status::W) writer structure"]
impl crate::Writable for OP_STATUS {}
#[doc = "Flash Operation Status"]
pub mod op_status;
#[doc = "Flash Controller Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Flash Controller Status"]
pub mod status;
#[doc = "Flash Controller Scratch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch](scratch) module"]
pub type SCRATCH = crate::Reg<u32, _SCRATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH;
#[doc = "`read()` method returns [scratch::R](scratch::R) reader structure"]
impl crate::Readable for SCRATCH {}
#[doc = "`write(|w| ..)` method takes [scratch::W](scratch::W) writer structure"]
impl crate::Writable for SCRATCH {}
#[doc = "Flash Controller Scratch"]
pub mod scratch;
#[doc = "Programmable depth where fifos should generate interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_lvl](fifo_lvl) module"]
pub type FIFO_LVL = crate::Reg<u32, _FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_LVL;
#[doc = "`read()` method returns [fifo_lvl::R](fifo_lvl::R) reader structure"]
impl crate::Readable for FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [fifo_lvl::W](fifo_lvl::W) writer structure"]
impl crate::Writable for FIFO_LVL {}
#[doc = "Programmable depth where fifos should generate interrupts"]
pub mod fifo_lvl;
#[doc = "Flash program fifo. The fifo is 16 entries of 4B flash words\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prog_fifo](prog_fifo) module"]
pub type PROG_FIFO = crate::Reg<u32, _PROG_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROG_FIFO;
#[doc = "`write(|w| ..)` method takes [prog_fifo::W](prog_fifo::W) writer structure"]
impl crate::Writable for PROG_FIFO {}
#[doc = "Flash program fifo. The fifo is 16 entries of 4B flash words"]
pub mod prog_fifo;
#[doc = "Flash read fifo. The fifo is 16 entries of 4B flash words\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_fifo](rd_fifo) module"]
pub type RD_FIFO = crate::Reg<u32, _RD_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_FIFO;
#[doc = "`read()` method returns [rd_fifo::R](rd_fifo::R) reader structure"]
impl crate::Readable for RD_FIFO {}
#[doc = "Flash read fifo. The fifo is 16 entries of 4B flash words"]
pub mod rd_fifo;
