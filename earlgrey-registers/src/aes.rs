#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key: KEY,
    #[doc = "0x20 - Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
    pub data_in: DATA_IN,
    #[doc = "0x30 - Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
    pub data_out: DATA_OUT,
    #[doc = "0x40 - Control Register. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to this register are ignored."]
    pub ctrl: CTRL,
    #[doc = "0x44 - Trigger Register"]
    pub trigger: TRIGGER,
    #[doc = "0x48 - Status Register"]
    pub status: STATUS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key0: self::key::KEY0,
    #[doc = "0x04 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key1: self::key::KEY1,
    #[doc = "0x08 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key2: self::key::KEY2,
    #[doc = "0x0c - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key3: self::key::KEY3,
    #[doc = "0x10 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key4: self::key::KEY4,
    #[doc = "0x14 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key5: self::key::KEY5,
    #[doc = "0x18 - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key6: self::key::KEY6,
    #[doc = "0x1c - Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
    pub key7: self::key::KEY7,
}
#[doc = r"Register block"]
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key;
#[doc = r"Register block"]
#[repr(C)]
pub struct DATA_IN {
    #[doc = "0x00 - Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
    pub data_in0: self::data_in::DATA_IN0,
    #[doc = "0x04 - Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
    pub data_in1: self::data_in::DATA_IN1,
    #[doc = "0x08 - Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
    pub data_in2: self::data_in::DATA_IN2,
    #[doc = "0x0c - Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
    pub data_in3: self::data_in::DATA_IN3,
}
#[doc = r"Register block"]
#[doc = "Input Data Registers. Loaded into the internal State register upon starting encryption/decryption of the next block. After that, the processor can update the Input Data Register. The order in which the registers are written does not matter."]
pub mod data_in;
#[doc = r"Register block"]
#[repr(C)]
pub struct DATA_OUT {
    #[doc = "0x00 - Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
    pub data_out0: self::data_out::DATA_OUT0,
    #[doc = "0x04 - Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
    pub data_out1: self::data_out::DATA_OUT1,
    #[doc = "0x08 - Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
    pub data_out2: self::data_out::DATA_OUT2,
    #[doc = "0x0c - Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
    pub data_out3: self::data_out::DATA_OUT3,
}
#[doc = r"Register block"]
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
pub mod data_out;
#[doc = "Control Register. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to this register are ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to this register are ignored."]
pub mod ctrl;
#[doc = "Trigger Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](trigger) module"]
pub type TRIGGER = crate::Reg<u32, _TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGGER;
#[doc = "`write(|w| ..)` method takes [trigger::W](trigger::W) writer structure"]
impl crate::Writable for TRIGGER {}
#[doc = "Trigger Register"]
pub mod trigger;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
