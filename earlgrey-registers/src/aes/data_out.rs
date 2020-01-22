#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_out0](data_out0) module"]
pub type DATA_OUT0 = crate::Reg<u32, _DATA_OUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT0;
#[doc = "`read()` method returns [data_out0::R](data_out0::R) reader structure"]
impl crate::Readable for DATA_OUT0 {}
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
pub mod data_out0;
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_out1](data_out1) module"]
pub type DATA_OUT1 = crate::Reg<u32, _DATA_OUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT1;
#[doc = "`read()` method returns [data_out1::R](data_out1::R) reader structure"]
impl crate::Readable for DATA_OUT1 {}
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
pub mod data_out1;
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_out2](data_out2) module"]
pub type DATA_OUT2 = crate::Reg<u32, _DATA_OUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT2;
#[doc = "`read()` method returns [data_out2::R](data_out2::R) reader structure"]
impl crate::Readable for DATA_OUT2 {}
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
pub mod data_out2;
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_out3](data_out3) module"]
pub type DATA_OUT3 = crate::Reg<u32, _DATA_OUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT3;
#[doc = "`read()` method returns [data_out3::R](data_out3::R) reader structure"]
impl crate::Readable for DATA_OUT3 {}
#[doc = "Output Data Register. Holds the output data produced by the AES unit during the last encryption/decryption operation. If FORCE_DATA_OVERWRITE=0 (see Control Register), the AES unit is stalled when the previous output data has not yet been read and is about to be overwritten. The order in which the registers are read does not matter."]
pub mod data_out3;
