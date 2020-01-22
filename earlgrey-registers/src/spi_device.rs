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
    #[doc = "0x10 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x14 - RX/ TX FIFO levels."]
    pub fifo_level: FIFO_LEVEL,
    #[doc = "0x18 - RX/ TX Async FIFO levels between main clk and spi clock"]
    pub async_fifo_level: ASYNC_FIFO_LEVEL,
    #[doc = "0x1c - SPI Device status register"]
    pub status: STATUS,
    #[doc = "0x20 - Receiver FIFO (SRAM) pointers"]
    pub rxf_ptr: RXF_PTR,
    #[doc = "0x24 - Transmitter FIFO (SRAM) pointers"]
    pub txf_ptr: TXF_PTR,
    #[doc = "0x28 - Receiver FIFO (SRAM) Addresses"]
    pub rxf_addr: RXF_ADDR,
    #[doc = "0x2c - Transmitter FIFO (SRAM) Addresses"]
    pub txf_addr: TXF_ADDR,
    _reserved12: [u8; 2000usize],
    #[doc = "0x800 - SPI internal 2kB buffer. This buffer is shared by RX and TX circular buffer together."]
    pub buffer: [BUFFER; 512],
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
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "RX/ TX FIFO levels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_level](fifo_level) module"]
pub type FIFO_LEVEL = crate::Reg<u32, _FIFO_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_LEVEL;
#[doc = "`read()` method returns [fifo_level::R](fifo_level::R) reader structure"]
impl crate::Readable for FIFO_LEVEL {}
#[doc = "`write(|w| ..)` method takes [fifo_level::W](fifo_level::W) writer structure"]
impl crate::Writable for FIFO_LEVEL {}
#[doc = "RX/ TX FIFO levels."]
pub mod fifo_level;
#[doc = "RX/ TX Async FIFO levels between main clk and spi clock\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [async_fifo_level](async_fifo_level) module"]
pub type ASYNC_FIFO_LEVEL = crate::Reg<u32, _ASYNC_FIFO_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNC_FIFO_LEVEL;
#[doc = "`read()` method returns [async_fifo_level::R](async_fifo_level::R) reader structure"]
impl crate::Readable for ASYNC_FIFO_LEVEL {}
#[doc = "RX/ TX Async FIFO levels between main clk and spi clock"]
pub mod async_fifo_level;
#[doc = "SPI Device status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "SPI Device status register"]
pub mod status;
#[doc = "Receiver FIFO (SRAM) pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf_ptr](rxf_ptr) module"]
pub type RXF_PTR = crate::Reg<u32, _RXF_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF_PTR;
#[doc = "`read()` method returns [rxf_ptr::R](rxf_ptr::R) reader structure"]
impl crate::Readable for RXF_PTR {}
#[doc = "`write(|w| ..)` method takes [rxf_ptr::W](rxf_ptr::W) writer structure"]
impl crate::Writable for RXF_PTR {}
#[doc = "Receiver FIFO (SRAM) pointers"]
pub mod rxf_ptr;
#[doc = "Transmitter FIFO (SRAM) pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf_ptr](txf_ptr) module"]
pub type TXF_PTR = crate::Reg<u32, _TXF_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF_PTR;
#[doc = "`read()` method returns [txf_ptr::R](txf_ptr::R) reader structure"]
impl crate::Readable for TXF_PTR {}
#[doc = "`write(|w| ..)` method takes [txf_ptr::W](txf_ptr::W) writer structure"]
impl crate::Writable for TXF_PTR {}
#[doc = "Transmitter FIFO (SRAM) pointers"]
pub mod txf_ptr;
#[doc = "Receiver FIFO (SRAM) Addresses\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf_addr](rxf_addr) module"]
pub type RXF_ADDR = crate::Reg<u32, _RXF_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF_ADDR;
#[doc = "`read()` method returns [rxf_addr::R](rxf_addr::R) reader structure"]
impl crate::Readable for RXF_ADDR {}
#[doc = "`write(|w| ..)` method takes [rxf_addr::W](rxf_addr::W) writer structure"]
impl crate::Writable for RXF_ADDR {}
#[doc = "Receiver FIFO (SRAM) Addresses"]
pub mod rxf_addr;
#[doc = "Transmitter FIFO (SRAM) Addresses\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf_addr](txf_addr) module"]
pub type TXF_ADDR = crate::Reg<u32, _TXF_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXF_ADDR;
#[doc = "`read()` method returns [txf_addr::R](txf_addr::R) reader structure"]
impl crate::Readable for TXF_ADDR {}
#[doc = "`write(|w| ..)` method takes [txf_addr::W](txf_addr::W) writer structure"]
impl crate::Writable for TXF_ADDR {}
#[doc = "Transmitter FIFO (SRAM) Addresses"]
pub mod txf_addr;
#[doc = "SPI internal 2kB buffer. This buffer is shared by RX and TX circular buffer together.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffer](buffer) module"]
pub type BUFFER = crate::Reg<u32, _BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFFER;
#[doc = "`read()` method returns [buffer::R](buffer::R) reader structure"]
impl crate::Readable for BUFFER {}
#[doc = "`write(|w| ..)` method takes [buffer::W](buffer::W) writer structure"]
impl crate::Writable for BUFFER {}
#[doc = "SPI internal 2kB buffer. This buffer is shared by RX and TX circular buffer together."]
pub mod buffer;
