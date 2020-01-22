#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0](key0) module"]
pub type KEY0 = crate::Reg<u32, _KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY0;
#[doc = "`write(|w| ..)` method takes [key0::W](key0::W) writer structure"]
impl crate::Writable for KEY0 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key0;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key1;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key2;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key3](key3) module"]
pub type KEY3 = crate::Reg<u32, _KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY3;
#[doc = "`write(|w| ..)` method takes [key3::W](key3::W) writer structure"]
impl crate::Writable for KEY3 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key3;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key4](key4) module"]
pub type KEY4 = crate::Reg<u32, _KEY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY4;
#[doc = "`write(|w| ..)` method takes [key4::W](key4::W) writer structure"]
impl crate::Writable for KEY4 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key4;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key5](key5) module"]
pub type KEY5 = crate::Reg<u32, _KEY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY5;
#[doc = "`write(|w| ..)` method takes [key5::W](key5::W) writer structure"]
impl crate::Writable for KEY5 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key5;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key6](key6) module"]
pub type KEY6 = crate::Reg<u32, _KEY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY6;
#[doc = "`write(|w| ..)` method takes [key6::W](key6::W) writer structure"]
impl crate::Writable for KEY6 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key6;
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key7](key7) module"]
pub type KEY7 = crate::Reg<u32, _KEY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY7;
#[doc = "`write(|w| ..)` method takes [key7::W](key7::W) writer structure"]
impl crate::Writable for KEY7 {}
#[doc = "Initial Key Registers. Loaded into the internal Full Key register upon starting encryption/decryption of the next block. Can only be updated when the AES unit is idle. If the AES unit is non-idle, writes to these registers are ignored. All key registers must be updated when the key is changed, regardless of key length (write 0 for unusued bits). The order in which the registers are updated does not matter."]
pub mod key7;
