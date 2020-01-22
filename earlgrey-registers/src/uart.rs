#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
    #[doc = "0x0c - UART control register"]
    pub ctrl: CTRL,
    #[doc = "0x10 - UART live status register"]
    pub status: STATUS,
    #[doc = "0x14 - UART read data"]
    pub rdata: RDATA,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - UART write data"]
    pub wdata: WDATA,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - UART FIFO control register"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x20 - UART FIFO status register"]
    pub fifo_status: FIFO_STATUS,
    #[doc = "0x24 - TX pin override control. Gives direct SW control over TX pin state"]
    pub ovrd: OVRD,
    #[doc = "0x28 - UART oversampled values"]
    pub val: VAL,
    #[doc = "0x2c - UART RX timeout control"]
    pub timeout_ctrl: TIMEOUT_CTRL,
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
#[doc = "UART control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "UART control register"]
pub mod ctrl;
#[doc = "UART live status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "UART live status register"]
pub mod status;
#[doc = "UART read data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](rdata) module"]
pub type RDATA = crate::Reg<u8, _RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA;
#[doc = "`read()` method returns [rdata::R](rdata::R) reader structure"]
impl crate::Readable for RDATA {}
#[doc = "UART read data"]
pub mod rdata;
#[doc = "UART write data\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdata](wdata) module"]
pub type WDATA = crate::Reg<u8, _WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATA;
#[doc = "`write(|w| ..)` method takes [wdata::W](wdata::W) writer structure"]
impl crate::Writable for WDATA {}
#[doc = "UART write data"]
pub mod wdata;
#[doc = "UART FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](fifo_ctrl) module"]
pub type FIFO_CTRL = crate::Reg<u32, _FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CTRL;
#[doc = "`read()` method returns [fifo_ctrl::R](fifo_ctrl::R) reader structure"]
impl crate::Readable for FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](fifo_ctrl::W) writer structure"]
impl crate::Writable for FIFO_CTRL {}
#[doc = "UART FIFO control register"]
pub mod fifo_ctrl;
#[doc = "UART FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_status](fifo_status) module"]
pub type FIFO_STATUS = crate::Reg<u32, _FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_STATUS;
#[doc = "`read()` method returns [fifo_status::R](fifo_status::R) reader structure"]
impl crate::Readable for FIFO_STATUS {}
#[doc = "UART FIFO status register"]
pub mod fifo_status;
#[doc = "TX pin override control. Gives direct SW control over TX pin state\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrd](ovrd) module"]
pub type OVRD = crate::Reg<u32, _OVRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRD;
#[doc = "`read()` method returns [ovrd::R](ovrd::R) reader structure"]
impl crate::Readable for OVRD {}
#[doc = "`write(|w| ..)` method takes [ovrd::W](ovrd::W) writer structure"]
impl crate::Writable for OVRD {}
#[doc = "TX pin override control. Gives direct SW control over TX pin state"]
pub mod ovrd;
#[doc = "UART oversampled values\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [val](val) module"]
pub type VAL = crate::Reg<u32, _VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VAL;
#[doc = "`read()` method returns [val::R](val::R) reader structure"]
impl crate::Readable for VAL {}
#[doc = "UART oversampled values"]
pub mod val;
#[doc = "UART RX timeout control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_ctrl](timeout_ctrl) module"]
pub type TIMEOUT_CTRL = crate::Reg<u32, _TIMEOUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT_CTRL;
#[doc = "`read()` method returns [timeout_ctrl::R](timeout_ctrl::R) reader structure"]
impl crate::Readable for TIMEOUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [timeout_ctrl::W](timeout_ctrl::W) writer structure"]
impl crate::Writable for TIMEOUT_CTRL {}
#[doc = "UART RX timeout control"]
pub mod timeout_ctrl;
