#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt State Register"]
    pub intr_state: INTR_STATE,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub intr_enable: INTR_ENABLE,
    #[doc = "0x08 - Interrupt Test Register"]
    pub intr_test: INTR_TEST,
    #[doc = "0x0c - HMAC Configuration register. The register is updated when the engine is in Idle. If the software updates the register while the engine computes the hash, the updated value is discarded."]
    pub cfg: CFG,
    #[doc = "0x10 - HMAC command register"]
    pub cmd: CMD,
    #[doc = "0x14 - HMAC Status register"]
    pub status: STATUS,
    #[doc = "0x18 - HMAC Error Code"]
    pub err_code: ERR_CODE,
    #[doc = "0x1c - Randomize internal secret registers. If CPU writes value into the register, the value is used to randomize internal variables such as secret key, internal state machine, or hash value."]
    pub wipe_secret: WIPE_SECRET,
    #[doc = "0x20 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key: KEY,
    #[doc = "0x40 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest: DIGEST,
    #[doc = "0x60 - Received Message Length calculated by the HMAC in bits \\[31:0\\]
Message is byte granularity. lower 3bits \\[2:0\\]
are ignored."]
    pub msg_length_lower: MSG_LENGTH_LOWER,
    #[doc = "0x64 - Received Message Length calculated by the HMAC in bits \\[63:32\\]"]
    pub msg_length_upper: MSG_LENGTH_UPPER,
    _reserved12: [u8; 1944usize],
    #[doc = "0x800 - Message FIFO. Any write to this window will be appended to the FIFO. Only the lower \\[1:0\\]
bits of the address matter to writes within the window (for correctly dealing with non 32-bit writes)"]
    pub msg_fifo: [MSG_FIFO; 512],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key0: self::key::KEY0,
    #[doc = "0x04 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key1: self::key::KEY1,
    #[doc = "0x08 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key2: self::key::KEY2,
    #[doc = "0x0c - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key3: self::key::KEY3,
    #[doc = "0x10 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key4: self::key::KEY4,
    #[doc = "0x14 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key5: self::key::KEY5,
    #[doc = "0x18 - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key6: self::key::KEY6,
    #[doc = "0x1c - HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
    pub key7: self::key::KEY7,
}
#[doc = r"Register block"]
#[doc = "HMAC Secret Key SHA256 assumes secret key is hashed 256bit key. Order of the secret key is: key\\[255:0\\]
= {KEY0, KEY1, KEY2, ... , KEY7}; The registers are allowed to be updated when the engine is in Idle state. If the engine computes the hash, it discards any attempts to update the secret keys and report an error."]
pub mod key;
#[doc = r"Register block"]
#[repr(C)]
pub struct DIGEST {
    #[doc = "0x00 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest0: self::digest::DIGEST0,
    #[doc = "0x04 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest1: self::digest::DIGEST1,
    #[doc = "0x08 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest2: self::digest::DIGEST2,
    #[doc = "0x0c - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest3: self::digest::DIGEST3,
    #[doc = "0x10 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest4: self::digest::DIGEST4,
    #[doc = "0x14 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest5: self::digest::DIGEST5,
    #[doc = "0x18 - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest6: self::digest::DIGEST6,
    #[doc = "0x1c - Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
    pub digest7: self::digest::DIGEST7,
}
#[doc = r"Register block"]
#[doc = "Digest output. If HMAC is disabled, the register shows result of SHA256 Order of the digest is: digest\\[255:0\\]
= {DIGEST0, DIGEST1, DIGEST2, ... , DIGEST7};"]
pub mod digest;
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
#[doc = "HMAC Configuration register. The register is updated when the engine is in Idle. If the software updates the register while the engine computes the hash, the updated value is discarded.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "HMAC Configuration register. The register is updated when the engine is in Idle. If the software updates the register while the engine computes the hash, the updated value is discarded."]
pub mod cfg;
#[doc = "HMAC command register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "HMAC command register"]
pub mod cmd;
#[doc = "HMAC Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "HMAC Status register"]
pub mod status;
#[doc = "HMAC Error Code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_code](err_code) module"]
pub type ERR_CODE = crate::Reg<u32, _ERR_CODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR_CODE;
#[doc = "`read()` method returns [err_code::R](err_code::R) reader structure"]
impl crate::Readable for ERR_CODE {}
#[doc = "HMAC Error Code"]
pub mod err_code;
#[doc = "Randomize internal secret registers. If CPU writes value into the register, the value is used to randomize internal variables such as secret key, internal state machine, or hash value.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wipe_secret](wipe_secret) module"]
pub type WIPE_SECRET = crate::Reg<u32, _WIPE_SECRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIPE_SECRET;
#[doc = "`write(|w| ..)` method takes [wipe_secret::W](wipe_secret::W) writer structure"]
impl crate::Writable for WIPE_SECRET {}
#[doc = "Randomize internal secret registers. If CPU writes value into the register, the value is used to randomize internal variables such as secret key, internal state machine, or hash value."]
pub mod wipe_secret;
#[doc = "Received Message Length calculated by the HMAC in bits \\[31:0\\]
Message is byte granularity. lower 3bits \\[2:0\\]
are ignored.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg_length_lower](msg_length_lower) module"]
pub type MSG_LENGTH_LOWER = crate::Reg<u32, _MSG_LENGTH_LOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG_LENGTH_LOWER;
#[doc = "`read()` method returns [msg_length_lower::R](msg_length_lower::R) reader structure"]
impl crate::Readable for MSG_LENGTH_LOWER {}
#[doc = "Received Message Length calculated by the HMAC in bits \\[31:0\\]
Message is byte granularity. lower 3bits \\[2:0\\]
are ignored."]
pub mod msg_length_lower;
#[doc = "Received Message Length calculated by the HMAC in bits \\[63:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg_length_upper](msg_length_upper) module"]
pub type MSG_LENGTH_UPPER = crate::Reg<u32, _MSG_LENGTH_UPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG_LENGTH_UPPER;
#[doc = "`read()` method returns [msg_length_upper::R](msg_length_upper::R) reader structure"]
impl crate::Readable for MSG_LENGTH_UPPER {}
#[doc = "Received Message Length calculated by the HMAC in bits \\[63:32\\]"]
pub mod msg_length_upper;
#[doc = "Message FIFO. Any write to this window will be appended to the FIFO. Only the lower \\[1:0\\]
bits of the address matter to writes within the window (for correctly dealing with non 32-bit writes)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg_fifo](msg_fifo) module"]
pub type MSG_FIFO = crate::Reg<u32, _MSG_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG_FIFO;
#[doc = "`write(|w| ..)` method takes [msg_fifo::W](msg_fifo::W) writer structure"]
impl crate::Writable for MSG_FIFO {}
#[doc = "Message FIFO. Any write to this window will be appended to the FIFO. Only the lower \\[1:0\\]
bits of the address matter to writes within the window (for correctly dealing with non 32-bit writes)"]
pub mod msg_fifo;
