#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
    #[doc = "0x0c - GPIO Input data read value"]
    pub data_in: DATA_IN,
    #[doc = "0x10 - GPIO direct output data write value"]
    pub direct_out: DIRECT_OUT,
    #[doc = "0x14 - GPIO write data lower with mask. Masked write for DATA_OUT\\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[15:0\\]."]
    pub masked_out_lower: MASKED_OUT_LOWER,
    #[doc = "0x18 - GPIO write data upper with mask. Masked write for DATA_OUT\\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[31:16\\]."]
    pub masked_out_upper: MASKED_OUT_UPPER,
    #[doc = "0x1c - GPIO Output Enable. Setting direct_oe\\[i\\]
to 1 enables output mode for GPIO\\[i\\]"]
    pub direct_oe: DIRECT_OE,
    #[doc = "0x20 - GPIO write Output Enable lower with mask. Masked write for DATA_OE\\[15:0\\], the register that controls output mode for GPIO pins \\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[15:0\\]."]
    pub masked_oe_lower: MASKED_OE_LOWER,
    #[doc = "0x24 - GPIO write Output Enable upper with mask. Masked write for DATA_OE\\[31:16\\], the register that controls output mode for GPIO pins \\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[31:16\\]."]
    pub masked_oe_upper: MASKED_OE_UPPER,
    #[doc = "0x28 - GPIO interrupt enable for GPIO, rising edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_RISING\\[i\\]
enables rising-edge interrupt detection on GPIO\\[i\\]."]
    pub intr_ctrl_en_rising: INTR_CTRL_EN_RISING,
    #[doc = "0x2c - GPIO interrupt enable for GPIO, falling edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_FALLING\\[i\\]
enables falling-edge interrupt detection on GPIO\\[i\\]."]
    pub intr_ctrl_en_falling: INTR_CTRL_EN_FALLING,
    #[doc = "0x30 - GPIO interrupt enable for GPIO, level high. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLHIGH\\[i\\]
enables level high interrupt detection on GPIO\\[i\\]."]
    pub intr_ctrl_en_lvlhigh: INTR_CTRL_EN_LVLHIGH,
    #[doc = "0x34 - GPIO interrupt enable for GPIO, level low. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLLOW\\[i\\]
enables level low interrupt detection on GPIO\\[i\\]."]
    pub intr_ctrl_en_lvllow: INTR_CTRL_EN_LVLLOW,
    #[doc = "0x38 - filter enable for GPIO input bits. If !!CTRL_EN_INPUT_FILTER\\[i\\]
is true, a value of input bit \\[i\\]
must be stable for 16 cycles before transitioning."]
    pub ctrl_en_input_filter: CTRL_EN_INPUT_FILTER,
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
#[doc = "GPIO Input data read value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in](data_in) module"]
pub type DATA_IN = crate::Reg<u32, _DATA_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN;
#[doc = "`read()` method returns [data_in::R](data_in::R) reader structure"]
impl crate::Readable for DATA_IN {}
#[doc = "GPIO Input data read value"]
pub mod data_in;
#[doc = "GPIO direct output data write value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [direct_out](direct_out) module"]
pub type DIRECT_OUT = crate::Reg<u32, _DIRECT_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRECT_OUT;
#[doc = "`read()` method returns [direct_out::R](direct_out::R) reader structure"]
impl crate::Readable for DIRECT_OUT {}
#[doc = "`write(|w| ..)` method takes [direct_out::W](direct_out::W) writer structure"]
impl crate::Writable for DIRECT_OUT {}
#[doc = "GPIO direct output data write value"]
pub mod direct_out;
#[doc = "GPIO write data lower with mask. Masked write for DATA_OUT\\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[15:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masked_out_lower](masked_out_lower) module"]
pub type MASKED_OUT_LOWER = crate::Reg<u32, _MASKED_OUT_LOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKED_OUT_LOWER;
#[doc = "`read()` method returns [masked_out_lower::R](masked_out_lower::R) reader structure"]
impl crate::Readable for MASKED_OUT_LOWER {}
#[doc = "`write(|w| ..)` method takes [masked_out_lower::W](masked_out_lower::W) writer structure"]
impl crate::Writable for MASKED_OUT_LOWER {}
#[doc = "GPIO write data lower with mask. Masked write for DATA_OUT\\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[15:0\\]."]
pub mod masked_out_lower;
#[doc = "GPIO write data upper with mask. Masked write for DATA_OUT\\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[31:16\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masked_out_upper](masked_out_upper) module"]
pub type MASKED_OUT_UPPER = crate::Reg<u32, _MASKED_OUT_UPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKED_OUT_UPPER;
#[doc = "`read()` method returns [masked_out_upper::R](masked_out_upper::R) reader structure"]
impl crate::Readable for MASKED_OUT_UPPER {}
#[doc = "`write(|w| ..)` method takes [masked_out_upper::W](masked_out_upper::W) writer structure"]
impl crate::Writable for MASKED_OUT_UPPER {}
#[doc = "GPIO write data upper with mask. Masked write for DATA_OUT\\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OUT\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OUT\\[31:16\\]."]
pub mod masked_out_upper;
#[doc = "GPIO Output Enable. Setting direct_oe\\[i\\]
to 1 enables output mode for GPIO\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [direct_oe](direct_oe) module"]
pub type DIRECT_OE = crate::Reg<u32, _DIRECT_OE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRECT_OE;
#[doc = "`read()` method returns [direct_oe::R](direct_oe::R) reader structure"]
impl crate::Readable for DIRECT_OE {}
#[doc = "`write(|w| ..)` method takes [direct_oe::W](direct_oe::W) writer structure"]
impl crate::Writable for DIRECT_OE {}
#[doc = "GPIO Output Enable. Setting direct_oe\\[i\\]
to 1 enables output mode for GPIO\\[i\\]"]
pub mod direct_oe;
#[doc = "GPIO write Output Enable lower with mask. Masked write for DATA_OE\\[15:0\\], the register that controls output mode for GPIO pins \\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[15:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masked_oe_lower](masked_oe_lower) module"]
pub type MASKED_OE_LOWER = crate::Reg<u32, _MASKED_OE_LOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKED_OE_LOWER;
#[doc = "`read()` method returns [masked_oe_lower::R](masked_oe_lower::R) reader structure"]
impl crate::Readable for MASKED_OE_LOWER {}
#[doc = "`write(|w| ..)` method takes [masked_oe_lower::W](masked_oe_lower::W) writer structure"]
impl crate::Writable for MASKED_OE_LOWER {}
#[doc = "GPIO write Output Enable lower with mask. Masked write for DATA_OE\\[15:0\\], the register that controls output mode for GPIO pins \\[15:0\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[15:0\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[15:0\\]."]
pub mod masked_oe_lower;
#[doc = "GPIO write Output Enable upper with mask. Masked write for DATA_OE\\[31:16\\], the register that controls output mode for GPIO pins \\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[31:16\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masked_oe_upper](masked_oe_upper) module"]
pub type MASKED_OE_UPPER = crate::Reg<u32, _MASKED_OE_UPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKED_OE_UPPER;
#[doc = "`read()` method returns [masked_oe_upper::R](masked_oe_upper::R) reader structure"]
impl crate::Readable for MASKED_OE_UPPER {}
#[doc = "`write(|w| ..)` method takes [masked_oe_upper::W](masked_oe_upper::W) writer structure"]
impl crate::Writable for MASKED_OE_UPPER {}
#[doc = "GPIO write Output Enable upper with mask. Masked write for DATA_OE\\[31:16\\], the register that controls output mode for GPIO pins \\[31:16\\]. Upper 16 bits of this register are used as mask. Writing lower 16 bits of the register changes DATA_OE\\[31:16\\]
value if mask bits are set. Read-back of this register returns upper 16 bits as zero and lower 16 bits as DATA_OE\\[31:16\\]."]
pub mod masked_oe_upper;
#[doc = "GPIO interrupt enable for GPIO, rising edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_RISING\\[i\\]
enables rising-edge interrupt detection on GPIO\\[i\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ctrl_en_rising](intr_ctrl_en_rising) module"]
pub type INTR_CTRL_EN_RISING = crate::Reg<u32, _INTR_CTRL_EN_RISING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CTRL_EN_RISING;
#[doc = "`read()` method returns [intr_ctrl_en_rising::R](intr_ctrl_en_rising::R) reader structure"]
impl crate::Readable for INTR_CTRL_EN_RISING {}
#[doc = "`write(|w| ..)` method takes [intr_ctrl_en_rising::W](intr_ctrl_en_rising::W) writer structure"]
impl crate::Writable for INTR_CTRL_EN_RISING {}
#[doc = "GPIO interrupt enable for GPIO, rising edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_RISING\\[i\\]
enables rising-edge interrupt detection on GPIO\\[i\\]."]
pub mod intr_ctrl_en_rising;
#[doc = "GPIO interrupt enable for GPIO, falling edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_FALLING\\[i\\]
enables falling-edge interrupt detection on GPIO\\[i\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ctrl_en_falling](intr_ctrl_en_falling) module"]
pub type INTR_CTRL_EN_FALLING = crate::Reg<u32, _INTR_CTRL_EN_FALLING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CTRL_EN_FALLING;
#[doc = "`read()` method returns [intr_ctrl_en_falling::R](intr_ctrl_en_falling::R) reader structure"]
impl crate::Readable for INTR_CTRL_EN_FALLING {}
#[doc = "`write(|w| ..)` method takes [intr_ctrl_en_falling::W](intr_ctrl_en_falling::W) writer structure"]
impl crate::Writable for INTR_CTRL_EN_FALLING {}
#[doc = "GPIO interrupt enable for GPIO, falling edge. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_FALLING\\[i\\]
enables falling-edge interrupt detection on GPIO\\[i\\]."]
pub mod intr_ctrl_en_falling;
#[doc = "GPIO interrupt enable for GPIO, level high. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLHIGH\\[i\\]
enables level high interrupt detection on GPIO\\[i\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ctrl_en_lvlhigh](intr_ctrl_en_lvlhigh) module"]
pub type INTR_CTRL_EN_LVLHIGH = crate::Reg<u32, _INTR_CTRL_EN_LVLHIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CTRL_EN_LVLHIGH;
#[doc = "`read()` method returns [intr_ctrl_en_lvlhigh::R](intr_ctrl_en_lvlhigh::R) reader structure"]
impl crate::Readable for INTR_CTRL_EN_LVLHIGH {}
#[doc = "`write(|w| ..)` method takes [intr_ctrl_en_lvlhigh::W](intr_ctrl_en_lvlhigh::W) writer structure"]
impl crate::Writable for INTR_CTRL_EN_LVLHIGH {}
#[doc = "GPIO interrupt enable for GPIO, level high. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLHIGH\\[i\\]
enables level high interrupt detection on GPIO\\[i\\]."]
pub mod intr_ctrl_en_lvlhigh;
#[doc = "GPIO interrupt enable for GPIO, level low. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLLOW\\[i\\]
enables level low interrupt detection on GPIO\\[i\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ctrl_en_lvllow](intr_ctrl_en_lvllow) module"]
pub type INTR_CTRL_EN_LVLLOW = crate::Reg<u32, _INTR_CTRL_EN_LVLLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CTRL_EN_LVLLOW;
#[doc = "`read()` method returns [intr_ctrl_en_lvllow::R](intr_ctrl_en_lvllow::R) reader structure"]
impl crate::Readable for INTR_CTRL_EN_LVLLOW {}
#[doc = "`write(|w| ..)` method takes [intr_ctrl_en_lvllow::W](intr_ctrl_en_lvllow::W) writer structure"]
impl crate::Writable for INTR_CTRL_EN_LVLLOW {}
#[doc = "GPIO interrupt enable for GPIO, level low. If !!INTR_ENABLE\\[i\\]
is true, a value of 1 on !!INTR_CTRL_EN_LVLLOW\\[i\\]
enables level low interrupt detection on GPIO\\[i\\]."]
pub mod intr_ctrl_en_lvllow;
#[doc = "filter enable for GPIO input bits. If !!CTRL_EN_INPUT_FILTER\\[i\\]
is true, a value of input bit \\[i\\]
must be stable for 16 cycles before transitioning.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_en_input_filter](ctrl_en_input_filter) module"]
pub type CTRL_EN_INPUT_FILTER = crate::Reg<u32, _CTRL_EN_INPUT_FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_EN_INPUT_FILTER;
#[doc = "`read()` method returns [ctrl_en_input_filter::R](ctrl_en_input_filter::R) reader structure"]
impl crate::Readable for CTRL_EN_INPUT_FILTER {}
#[doc = "`write(|w| ..)` method takes [ctrl_en_input_filter::W](ctrl_en_input_filter::W) writer structure"]
impl crate::Writable for CTRL_EN_INPUT_FILTER {}
#[doc = "filter enable for GPIO input bits. If !!CTRL_EN_INPUT_FILTER\\[i\\]
is true, a value of input bit \\[i\\]
must be stable for 16 cycles before transitioning."]
pub mod ctrl_en_input_filter;
