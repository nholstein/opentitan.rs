#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Pending"]
    pub ip: IP,
    #[doc = "0x08 - Interrupt Source mode. 0: Level, 1: Edge-triggered"]
    pub le: LE,
    #[doc = "0x10 - Interrupt Source 0 Priority"]
    pub prio0: PRIO0,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Interrupt Source 1 Priority"]
    pub prio1: PRIO1,
    _reserved4: [u8; 4usize],
    #[doc = "0x18 - Interrupt Source 2 Priority"]
    pub prio2: PRIO2,
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - Interrupt Source 3 Priority"]
    pub prio3: PRIO3,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Interrupt Source 4 Priority"]
    pub prio4: PRIO4,
    _reserved7: [u8; 4usize],
    #[doc = "0x24 - Interrupt Source 5 Priority"]
    pub prio5: PRIO5,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - Interrupt Source 6 Priority"]
    pub prio6: PRIO6,
    _reserved9: [u8; 4usize],
    #[doc = "0x2c - Interrupt Source 7 Priority"]
    pub prio7: PRIO7,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Interrupt Source 8 Priority"]
    pub prio8: PRIO8,
    _reserved11: [u8; 4usize],
    #[doc = "0x34 - Interrupt Source 9 Priority"]
    pub prio9: PRIO9,
    _reserved12: [u8; 4usize],
    #[doc = "0x38 - Interrupt Source 10 Priority"]
    pub prio10: PRIO10,
    _reserved13: [u8; 4usize],
    #[doc = "0x3c - Interrupt Source 11 Priority"]
    pub prio11: PRIO11,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - Interrupt Source 12 Priority"]
    pub prio12: PRIO12,
    _reserved15: [u8; 4usize],
    #[doc = "0x44 - Interrupt Source 13 Priority"]
    pub prio13: PRIO13,
    _reserved16: [u8; 4usize],
    #[doc = "0x48 - Interrupt Source 14 Priority"]
    pub prio14: PRIO14,
    _reserved17: [u8; 4usize],
    #[doc = "0x4c - Interrupt Source 15 Priority"]
    pub prio15: PRIO15,
    _reserved18: [u8; 4usize],
    #[doc = "0x50 - Interrupt Source 16 Priority"]
    pub prio16: PRIO16,
    _reserved19: [u8; 4usize],
    #[doc = "0x54 - Interrupt Source 17 Priority"]
    pub prio17: PRIO17,
    _reserved20: [u8; 4usize],
    #[doc = "0x58 - Interrupt Source 18 Priority"]
    pub prio18: PRIO18,
    _reserved21: [u8; 4usize],
    #[doc = "0x5c - Interrupt Source 19 Priority"]
    pub prio19: PRIO19,
    _reserved22: [u8; 4usize],
    #[doc = "0x60 - Interrupt Source 20 Priority"]
    pub prio20: PRIO20,
    _reserved23: [u8; 4usize],
    #[doc = "0x64 - Interrupt Source 21 Priority"]
    pub prio21: PRIO21,
    _reserved24: [u8; 4usize],
    #[doc = "0x68 - Interrupt Source 22 Priority"]
    pub prio22: PRIO22,
    _reserved25: [u8; 4usize],
    #[doc = "0x6c - Interrupt Source 23 Priority"]
    pub prio23: PRIO23,
    _reserved26: [u8; 4usize],
    #[doc = "0x70 - Interrupt Source 24 Priority"]
    pub prio24: PRIO24,
    _reserved27: [u8; 4usize],
    #[doc = "0x74 - Interrupt Source 25 Priority"]
    pub prio25: PRIO25,
    _reserved28: [u8; 4usize],
    #[doc = "0x78 - Interrupt Source 26 Priority"]
    pub prio26: PRIO26,
    _reserved29: [u8; 4usize],
    #[doc = "0x7c - Interrupt Source 27 Priority"]
    pub prio27: PRIO27,
    _reserved30: [u8; 4usize],
    #[doc = "0x80 - Interrupt Source 28 Priority"]
    pub prio28: PRIO28,
    _reserved31: [u8; 4usize],
    #[doc = "0x84 - Interrupt Source 29 Priority"]
    pub prio29: PRIO29,
    _reserved32: [u8; 4usize],
    #[doc = "0x88 - Interrupt Source 30 Priority"]
    pub prio30: PRIO30,
    _reserved33: [u8; 4usize],
    #[doc = "0x8c - Interrupt Source 31 Priority"]
    pub prio31: PRIO31,
    _reserved34: [u8; 4usize],
    #[doc = "0x90 - Interrupt Source 32 Priority"]
    pub prio32: PRIO32,
    _reserved35: [u8; 4usize],
    #[doc = "0x94 - Interrupt Source 33 Priority"]
    pub prio33: PRIO33,
    _reserved36: [u8; 4usize],
    #[doc = "0x98 - Interrupt Source 34 Priority"]
    pub prio34: PRIO34,
    _reserved37: [u8; 4usize],
    #[doc = "0x9c - Interrupt Source 35 Priority"]
    pub prio35: PRIO35,
    _reserved38: [u8; 4usize],
    #[doc = "0xa0 - Interrupt Source 36 Priority"]
    pub prio36: PRIO36,
    _reserved39: [u8; 4usize],
    #[doc = "0xa4 - Interrupt Source 37 Priority"]
    pub prio37: PRIO37,
    _reserved40: [u8; 4usize],
    #[doc = "0xa8 - Interrupt Source 38 Priority"]
    pub prio38: PRIO38,
    _reserved41: [u8; 4usize],
    #[doc = "0xac - Interrupt Source 39 Priority"]
    pub prio39: PRIO39,
    _reserved42: [u8; 4usize],
    #[doc = "0xb0 - Interrupt Source 40 Priority"]
    pub prio40: PRIO40,
    _reserved43: [u8; 4usize],
    #[doc = "0xb4 - Interrupt Source 41 Priority"]
    pub prio41: PRIO41,
    _reserved44: [u8; 4usize],
    #[doc = "0xb8 - Interrupt Source 42 Priority"]
    pub prio42: PRIO42,
    _reserved45: [u8; 4usize],
    #[doc = "0xbc - Interrupt Source 43 Priority"]
    pub prio43: PRIO43,
    _reserved46: [u8; 4usize],
    #[doc = "0xc0 - Interrupt Source 44 Priority"]
    pub prio44: PRIO44,
    _reserved47: [u8; 4usize],
    #[doc = "0xc4 - Interrupt Source 45 Priority"]
    pub prio45: PRIO45,
    _reserved48: [u8; 4usize],
    #[doc = "0xc8 - Interrupt Source 46 Priority"]
    pub prio46: PRIO46,
    _reserved49: [u8; 4usize],
    #[doc = "0xcc - Interrupt Source 47 Priority"]
    pub prio47: PRIO47,
    _reserved50: [u8; 4usize],
    #[doc = "0xd0 - Interrupt Source 48 Priority"]
    pub prio48: PRIO48,
    _reserved51: [u8; 4usize],
    #[doc = "0xd4 - Interrupt Source 49 Priority"]
    pub prio49: PRIO49,
    _reserved52: [u8; 4usize],
    #[doc = "0xd8 - Interrupt Source 50 Priority"]
    pub prio50: PRIO50,
    _reserved53: [u8; 4usize],
    #[doc = "0xdc - Interrupt Source 51 Priority"]
    pub prio51: PRIO51,
    _reserved54: [u8; 4usize],
    #[doc = "0xe0 - Interrupt Source 52 Priority"]
    pub prio52: PRIO52,
    _reserved55: [u8; 4usize],
    #[doc = "0xe4 - Interrupt Source 53 Priority"]
    pub prio53: PRIO53,
    _reserved56: [u8; 4usize],
    #[doc = "0xe8 - Interrupt Source 54 Priority"]
    pub prio54: PRIO54,
    _reserved57: [u8; 4usize],
    #[doc = "0xec - Interrupt Source 55 Priority"]
    pub prio55: PRIO55,
    _reserved58: [u8; 4usize],
    #[doc = "0xf0 - Interrupt Source 56 Priority"]
    pub prio56: PRIO56,
    _reserved59: [u8; 4usize],
    #[doc = "0xf4 - Interrupt Source 57 Priority"]
    pub prio57: PRIO57,
    _reserved60: [u8; 4usize],
    #[doc = "0xf8 - Interrupt Source 58 Priority"]
    pub prio58: PRIO58,
    _reserved61: [u8; 4usize],
    #[doc = "0xfc - Interrupt Source 59 Priority"]
    pub prio59: PRIO59,
    _reserved62: [u8; 4usize],
    #[doc = "0x100 - Interrupt Source 60 Priority"]
    pub prio60: PRIO60,
    _reserved63: [u8; 4usize],
    #[doc = "0x104 - Interrupt Source 61 Priority"]
    pub prio61: PRIO61,
    _reserved64: [u8; 4usize],
    #[doc = "0x108 - Interrupt Source 62 Priority"]
    pub prio62: PRIO62,
    _reserved65: [u8; 248usize],
    #[doc = "0x200 - Interrupt Enable for Target 0"]
    pub ie0: IE0,
    #[doc = "0x208 - Threshold of priority for Target 0"]
    pub threshold0: THRESHOLD0,
    _reserved67: [u8; 4usize],
    #[doc = "0x20c - Claim interrupt by read, complete interrupt by write for Target 0. Value read/written is interrupt ID. Reading a value of 0 means no pending interrupts."]
    pub cc0: CC0,
    _reserved68: [u8; 4usize],
    #[doc = "0x210 - msip for Hart 0. Write 1 to here asserts software interrupt for Hart msip_o\\[0\\], write 0 to clear."]
    pub msip0: MSIP0,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct IP {
    #[doc = "0x00 - Interrupt Pending"]
    pub ip0: self::ip::IP0,
    #[doc = "0x04 - Interrupt Pending"]
    pub ip1: self::ip::IP1,
}
#[doc = r"Register block"]
#[doc = "Interrupt Pending"]
pub mod ip;
#[doc = r"Register block"]
#[repr(C)]
pub struct LE {
    #[doc = "0x00 - Interrupt Source mode. 0: Level, 1: Edge-triggered"]
    pub le0: self::le::LE0,
    #[doc = "0x04 - Interrupt Source mode. 0: Level, 1: Edge-triggered"]
    pub le1: self::le::LE1,
}
#[doc = r"Register block"]
#[doc = "Interrupt Source mode. 0: Level, 1: Edge-triggered"]
pub mod le;
#[doc = r"Register block"]
#[repr(C)]
pub struct IE0 {
    #[doc = "0x00 - Interrupt Enable for Target 0"]
    pub ie00: self::ie0::IE00,
    #[doc = "0x04 - Interrupt Enable for Target 0"]
    pub ie01: self::ie0::IE01,
}
#[doc = r"Register block"]
#[doc = "Interrupt Enable for Target 0"]
pub mod ie0;
#[doc = "Interrupt Source 0 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio0](prio0) module"]
pub type PRIO0 = crate::Reg<u8, _PRIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO0;
#[doc = "`read()` method returns [prio0::R](prio0::R) reader structure"]
impl crate::Readable for PRIO0 {}
#[doc = "`write(|w| ..)` method takes [prio0::W](prio0::W) writer structure"]
impl crate::Writable for PRIO0 {}
#[doc = "Interrupt Source 0 Priority"]
pub mod prio0;
#[doc = "Interrupt Source 1 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio1](prio1) module"]
pub type PRIO1 = crate::Reg<u8, _PRIO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO1;
#[doc = "`read()` method returns [prio1::R](prio1::R) reader structure"]
impl crate::Readable for PRIO1 {}
#[doc = "`write(|w| ..)` method takes [prio1::W](prio1::W) writer structure"]
impl crate::Writable for PRIO1 {}
#[doc = "Interrupt Source 1 Priority"]
pub mod prio1;
#[doc = "Interrupt Source 2 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio2](prio2) module"]
pub type PRIO2 = crate::Reg<u8, _PRIO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO2;
#[doc = "`read()` method returns [prio2::R](prio2::R) reader structure"]
impl crate::Readable for PRIO2 {}
#[doc = "`write(|w| ..)` method takes [prio2::W](prio2::W) writer structure"]
impl crate::Writable for PRIO2 {}
#[doc = "Interrupt Source 2 Priority"]
pub mod prio2;
#[doc = "Interrupt Source 3 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio3](prio3) module"]
pub type PRIO3 = crate::Reg<u8, _PRIO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO3;
#[doc = "`read()` method returns [prio3::R](prio3::R) reader structure"]
impl crate::Readable for PRIO3 {}
#[doc = "`write(|w| ..)` method takes [prio3::W](prio3::W) writer structure"]
impl crate::Writable for PRIO3 {}
#[doc = "Interrupt Source 3 Priority"]
pub mod prio3;
#[doc = "Interrupt Source 4 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio4](prio4) module"]
pub type PRIO4 = crate::Reg<u8, _PRIO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO4;
#[doc = "`read()` method returns [prio4::R](prio4::R) reader structure"]
impl crate::Readable for PRIO4 {}
#[doc = "`write(|w| ..)` method takes [prio4::W](prio4::W) writer structure"]
impl crate::Writable for PRIO4 {}
#[doc = "Interrupt Source 4 Priority"]
pub mod prio4;
#[doc = "Interrupt Source 5 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio5](prio5) module"]
pub type PRIO5 = crate::Reg<u8, _PRIO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO5;
#[doc = "`read()` method returns [prio5::R](prio5::R) reader structure"]
impl crate::Readable for PRIO5 {}
#[doc = "`write(|w| ..)` method takes [prio5::W](prio5::W) writer structure"]
impl crate::Writable for PRIO5 {}
#[doc = "Interrupt Source 5 Priority"]
pub mod prio5;
#[doc = "Interrupt Source 6 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio6](prio6) module"]
pub type PRIO6 = crate::Reg<u8, _PRIO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO6;
#[doc = "`read()` method returns [prio6::R](prio6::R) reader structure"]
impl crate::Readable for PRIO6 {}
#[doc = "`write(|w| ..)` method takes [prio6::W](prio6::W) writer structure"]
impl crate::Writable for PRIO6 {}
#[doc = "Interrupt Source 6 Priority"]
pub mod prio6;
#[doc = "Interrupt Source 7 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio7](prio7) module"]
pub type PRIO7 = crate::Reg<u8, _PRIO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO7;
#[doc = "`read()` method returns [prio7::R](prio7::R) reader structure"]
impl crate::Readable for PRIO7 {}
#[doc = "`write(|w| ..)` method takes [prio7::W](prio7::W) writer structure"]
impl crate::Writable for PRIO7 {}
#[doc = "Interrupt Source 7 Priority"]
pub mod prio7;
#[doc = "Interrupt Source 8 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio8](prio8) module"]
pub type PRIO8 = crate::Reg<u8, _PRIO8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO8;
#[doc = "`read()` method returns [prio8::R](prio8::R) reader structure"]
impl crate::Readable for PRIO8 {}
#[doc = "`write(|w| ..)` method takes [prio8::W](prio8::W) writer structure"]
impl crate::Writable for PRIO8 {}
#[doc = "Interrupt Source 8 Priority"]
pub mod prio8;
#[doc = "Interrupt Source 9 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio9](prio9) module"]
pub type PRIO9 = crate::Reg<u8, _PRIO9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO9;
#[doc = "`read()` method returns [prio9::R](prio9::R) reader structure"]
impl crate::Readable for PRIO9 {}
#[doc = "`write(|w| ..)` method takes [prio9::W](prio9::W) writer structure"]
impl crate::Writable for PRIO9 {}
#[doc = "Interrupt Source 9 Priority"]
pub mod prio9;
#[doc = "Interrupt Source 10 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio10](prio10) module"]
pub type PRIO10 = crate::Reg<u8, _PRIO10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO10;
#[doc = "`read()` method returns [prio10::R](prio10::R) reader structure"]
impl crate::Readable for PRIO10 {}
#[doc = "`write(|w| ..)` method takes [prio10::W](prio10::W) writer structure"]
impl crate::Writable for PRIO10 {}
#[doc = "Interrupt Source 10 Priority"]
pub mod prio10;
#[doc = "Interrupt Source 11 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio11](prio11) module"]
pub type PRIO11 = crate::Reg<u8, _PRIO11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO11;
#[doc = "`read()` method returns [prio11::R](prio11::R) reader structure"]
impl crate::Readable for PRIO11 {}
#[doc = "`write(|w| ..)` method takes [prio11::W](prio11::W) writer structure"]
impl crate::Writable for PRIO11 {}
#[doc = "Interrupt Source 11 Priority"]
pub mod prio11;
#[doc = "Interrupt Source 12 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio12](prio12) module"]
pub type PRIO12 = crate::Reg<u8, _PRIO12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO12;
#[doc = "`read()` method returns [prio12::R](prio12::R) reader structure"]
impl crate::Readable for PRIO12 {}
#[doc = "`write(|w| ..)` method takes [prio12::W](prio12::W) writer structure"]
impl crate::Writable for PRIO12 {}
#[doc = "Interrupt Source 12 Priority"]
pub mod prio12;
#[doc = "Interrupt Source 13 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio13](prio13) module"]
pub type PRIO13 = crate::Reg<u8, _PRIO13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO13;
#[doc = "`read()` method returns [prio13::R](prio13::R) reader structure"]
impl crate::Readable for PRIO13 {}
#[doc = "`write(|w| ..)` method takes [prio13::W](prio13::W) writer structure"]
impl crate::Writable for PRIO13 {}
#[doc = "Interrupt Source 13 Priority"]
pub mod prio13;
#[doc = "Interrupt Source 14 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio14](prio14) module"]
pub type PRIO14 = crate::Reg<u8, _PRIO14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO14;
#[doc = "`read()` method returns [prio14::R](prio14::R) reader structure"]
impl crate::Readable for PRIO14 {}
#[doc = "`write(|w| ..)` method takes [prio14::W](prio14::W) writer structure"]
impl crate::Writable for PRIO14 {}
#[doc = "Interrupt Source 14 Priority"]
pub mod prio14;
#[doc = "Interrupt Source 15 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio15](prio15) module"]
pub type PRIO15 = crate::Reg<u8, _PRIO15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO15;
#[doc = "`read()` method returns [prio15::R](prio15::R) reader structure"]
impl crate::Readable for PRIO15 {}
#[doc = "`write(|w| ..)` method takes [prio15::W](prio15::W) writer structure"]
impl crate::Writable for PRIO15 {}
#[doc = "Interrupt Source 15 Priority"]
pub mod prio15;
#[doc = "Interrupt Source 16 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio16](prio16) module"]
pub type PRIO16 = crate::Reg<u8, _PRIO16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO16;
#[doc = "`read()` method returns [prio16::R](prio16::R) reader structure"]
impl crate::Readable for PRIO16 {}
#[doc = "`write(|w| ..)` method takes [prio16::W](prio16::W) writer structure"]
impl crate::Writable for PRIO16 {}
#[doc = "Interrupt Source 16 Priority"]
pub mod prio16;
#[doc = "Interrupt Source 17 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio17](prio17) module"]
pub type PRIO17 = crate::Reg<u8, _PRIO17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO17;
#[doc = "`read()` method returns [prio17::R](prio17::R) reader structure"]
impl crate::Readable for PRIO17 {}
#[doc = "`write(|w| ..)` method takes [prio17::W](prio17::W) writer structure"]
impl crate::Writable for PRIO17 {}
#[doc = "Interrupt Source 17 Priority"]
pub mod prio17;
#[doc = "Interrupt Source 18 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio18](prio18) module"]
pub type PRIO18 = crate::Reg<u8, _PRIO18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO18;
#[doc = "`read()` method returns [prio18::R](prio18::R) reader structure"]
impl crate::Readable for PRIO18 {}
#[doc = "`write(|w| ..)` method takes [prio18::W](prio18::W) writer structure"]
impl crate::Writable for PRIO18 {}
#[doc = "Interrupt Source 18 Priority"]
pub mod prio18;
#[doc = "Interrupt Source 19 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio19](prio19) module"]
pub type PRIO19 = crate::Reg<u8, _PRIO19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO19;
#[doc = "`read()` method returns [prio19::R](prio19::R) reader structure"]
impl crate::Readable for PRIO19 {}
#[doc = "`write(|w| ..)` method takes [prio19::W](prio19::W) writer structure"]
impl crate::Writable for PRIO19 {}
#[doc = "Interrupt Source 19 Priority"]
pub mod prio19;
#[doc = "Interrupt Source 20 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio20](prio20) module"]
pub type PRIO20 = crate::Reg<u8, _PRIO20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO20;
#[doc = "`read()` method returns [prio20::R](prio20::R) reader structure"]
impl crate::Readable for PRIO20 {}
#[doc = "`write(|w| ..)` method takes [prio20::W](prio20::W) writer structure"]
impl crate::Writable for PRIO20 {}
#[doc = "Interrupt Source 20 Priority"]
pub mod prio20;
#[doc = "Interrupt Source 21 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio21](prio21) module"]
pub type PRIO21 = crate::Reg<u8, _PRIO21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO21;
#[doc = "`read()` method returns [prio21::R](prio21::R) reader structure"]
impl crate::Readable for PRIO21 {}
#[doc = "`write(|w| ..)` method takes [prio21::W](prio21::W) writer structure"]
impl crate::Writable for PRIO21 {}
#[doc = "Interrupt Source 21 Priority"]
pub mod prio21;
#[doc = "Interrupt Source 22 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio22](prio22) module"]
pub type PRIO22 = crate::Reg<u8, _PRIO22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO22;
#[doc = "`read()` method returns [prio22::R](prio22::R) reader structure"]
impl crate::Readable for PRIO22 {}
#[doc = "`write(|w| ..)` method takes [prio22::W](prio22::W) writer structure"]
impl crate::Writable for PRIO22 {}
#[doc = "Interrupt Source 22 Priority"]
pub mod prio22;
#[doc = "Interrupt Source 23 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio23](prio23) module"]
pub type PRIO23 = crate::Reg<u8, _PRIO23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO23;
#[doc = "`read()` method returns [prio23::R](prio23::R) reader structure"]
impl crate::Readable for PRIO23 {}
#[doc = "`write(|w| ..)` method takes [prio23::W](prio23::W) writer structure"]
impl crate::Writable for PRIO23 {}
#[doc = "Interrupt Source 23 Priority"]
pub mod prio23;
#[doc = "Interrupt Source 24 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio24](prio24) module"]
pub type PRIO24 = crate::Reg<u8, _PRIO24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO24;
#[doc = "`read()` method returns [prio24::R](prio24::R) reader structure"]
impl crate::Readable for PRIO24 {}
#[doc = "`write(|w| ..)` method takes [prio24::W](prio24::W) writer structure"]
impl crate::Writable for PRIO24 {}
#[doc = "Interrupt Source 24 Priority"]
pub mod prio24;
#[doc = "Interrupt Source 25 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio25](prio25) module"]
pub type PRIO25 = crate::Reg<u8, _PRIO25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO25;
#[doc = "`read()` method returns [prio25::R](prio25::R) reader structure"]
impl crate::Readable for PRIO25 {}
#[doc = "`write(|w| ..)` method takes [prio25::W](prio25::W) writer structure"]
impl crate::Writable for PRIO25 {}
#[doc = "Interrupt Source 25 Priority"]
pub mod prio25;
#[doc = "Interrupt Source 26 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio26](prio26) module"]
pub type PRIO26 = crate::Reg<u8, _PRIO26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO26;
#[doc = "`read()` method returns [prio26::R](prio26::R) reader structure"]
impl crate::Readable for PRIO26 {}
#[doc = "`write(|w| ..)` method takes [prio26::W](prio26::W) writer structure"]
impl crate::Writable for PRIO26 {}
#[doc = "Interrupt Source 26 Priority"]
pub mod prio26;
#[doc = "Interrupt Source 27 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio27](prio27) module"]
pub type PRIO27 = crate::Reg<u8, _PRIO27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO27;
#[doc = "`read()` method returns [prio27::R](prio27::R) reader structure"]
impl crate::Readable for PRIO27 {}
#[doc = "`write(|w| ..)` method takes [prio27::W](prio27::W) writer structure"]
impl crate::Writable for PRIO27 {}
#[doc = "Interrupt Source 27 Priority"]
pub mod prio27;
#[doc = "Interrupt Source 28 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio28](prio28) module"]
pub type PRIO28 = crate::Reg<u8, _PRIO28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO28;
#[doc = "`read()` method returns [prio28::R](prio28::R) reader structure"]
impl crate::Readable for PRIO28 {}
#[doc = "`write(|w| ..)` method takes [prio28::W](prio28::W) writer structure"]
impl crate::Writable for PRIO28 {}
#[doc = "Interrupt Source 28 Priority"]
pub mod prio28;
#[doc = "Interrupt Source 29 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio29](prio29) module"]
pub type PRIO29 = crate::Reg<u8, _PRIO29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO29;
#[doc = "`read()` method returns [prio29::R](prio29::R) reader structure"]
impl crate::Readable for PRIO29 {}
#[doc = "`write(|w| ..)` method takes [prio29::W](prio29::W) writer structure"]
impl crate::Writable for PRIO29 {}
#[doc = "Interrupt Source 29 Priority"]
pub mod prio29;
#[doc = "Interrupt Source 30 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio30](prio30) module"]
pub type PRIO30 = crate::Reg<u8, _PRIO30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO30;
#[doc = "`read()` method returns [prio30::R](prio30::R) reader structure"]
impl crate::Readable for PRIO30 {}
#[doc = "`write(|w| ..)` method takes [prio30::W](prio30::W) writer structure"]
impl crate::Writable for PRIO30 {}
#[doc = "Interrupt Source 30 Priority"]
pub mod prio30;
#[doc = "Interrupt Source 31 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio31](prio31) module"]
pub type PRIO31 = crate::Reg<u8, _PRIO31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO31;
#[doc = "`read()` method returns [prio31::R](prio31::R) reader structure"]
impl crate::Readable for PRIO31 {}
#[doc = "`write(|w| ..)` method takes [prio31::W](prio31::W) writer structure"]
impl crate::Writable for PRIO31 {}
#[doc = "Interrupt Source 31 Priority"]
pub mod prio31;
#[doc = "Interrupt Source 32 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio32](prio32) module"]
pub type PRIO32 = crate::Reg<u8, _PRIO32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO32;
#[doc = "`read()` method returns [prio32::R](prio32::R) reader structure"]
impl crate::Readable for PRIO32 {}
#[doc = "`write(|w| ..)` method takes [prio32::W](prio32::W) writer structure"]
impl crate::Writable for PRIO32 {}
#[doc = "Interrupt Source 32 Priority"]
pub mod prio32;
#[doc = "Interrupt Source 33 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio33](prio33) module"]
pub type PRIO33 = crate::Reg<u8, _PRIO33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO33;
#[doc = "`read()` method returns [prio33::R](prio33::R) reader structure"]
impl crate::Readable for PRIO33 {}
#[doc = "`write(|w| ..)` method takes [prio33::W](prio33::W) writer structure"]
impl crate::Writable for PRIO33 {}
#[doc = "Interrupt Source 33 Priority"]
pub mod prio33;
#[doc = "Interrupt Source 34 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio34](prio34) module"]
pub type PRIO34 = crate::Reg<u8, _PRIO34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO34;
#[doc = "`read()` method returns [prio34::R](prio34::R) reader structure"]
impl crate::Readable for PRIO34 {}
#[doc = "`write(|w| ..)` method takes [prio34::W](prio34::W) writer structure"]
impl crate::Writable for PRIO34 {}
#[doc = "Interrupt Source 34 Priority"]
pub mod prio34;
#[doc = "Interrupt Source 35 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio35](prio35) module"]
pub type PRIO35 = crate::Reg<u8, _PRIO35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO35;
#[doc = "`read()` method returns [prio35::R](prio35::R) reader structure"]
impl crate::Readable for PRIO35 {}
#[doc = "`write(|w| ..)` method takes [prio35::W](prio35::W) writer structure"]
impl crate::Writable for PRIO35 {}
#[doc = "Interrupt Source 35 Priority"]
pub mod prio35;
#[doc = "Interrupt Source 36 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio36](prio36) module"]
pub type PRIO36 = crate::Reg<u8, _PRIO36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO36;
#[doc = "`read()` method returns [prio36::R](prio36::R) reader structure"]
impl crate::Readable for PRIO36 {}
#[doc = "`write(|w| ..)` method takes [prio36::W](prio36::W) writer structure"]
impl crate::Writable for PRIO36 {}
#[doc = "Interrupt Source 36 Priority"]
pub mod prio36;
#[doc = "Interrupt Source 37 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio37](prio37) module"]
pub type PRIO37 = crate::Reg<u8, _PRIO37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO37;
#[doc = "`read()` method returns [prio37::R](prio37::R) reader structure"]
impl crate::Readable for PRIO37 {}
#[doc = "`write(|w| ..)` method takes [prio37::W](prio37::W) writer structure"]
impl crate::Writable for PRIO37 {}
#[doc = "Interrupt Source 37 Priority"]
pub mod prio37;
#[doc = "Interrupt Source 38 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio38](prio38) module"]
pub type PRIO38 = crate::Reg<u8, _PRIO38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO38;
#[doc = "`read()` method returns [prio38::R](prio38::R) reader structure"]
impl crate::Readable for PRIO38 {}
#[doc = "`write(|w| ..)` method takes [prio38::W](prio38::W) writer structure"]
impl crate::Writable for PRIO38 {}
#[doc = "Interrupt Source 38 Priority"]
pub mod prio38;
#[doc = "Interrupt Source 39 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio39](prio39) module"]
pub type PRIO39 = crate::Reg<u8, _PRIO39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO39;
#[doc = "`read()` method returns [prio39::R](prio39::R) reader structure"]
impl crate::Readable for PRIO39 {}
#[doc = "`write(|w| ..)` method takes [prio39::W](prio39::W) writer structure"]
impl crate::Writable for PRIO39 {}
#[doc = "Interrupt Source 39 Priority"]
pub mod prio39;
#[doc = "Interrupt Source 40 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio40](prio40) module"]
pub type PRIO40 = crate::Reg<u8, _PRIO40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO40;
#[doc = "`read()` method returns [prio40::R](prio40::R) reader structure"]
impl crate::Readable for PRIO40 {}
#[doc = "`write(|w| ..)` method takes [prio40::W](prio40::W) writer structure"]
impl crate::Writable for PRIO40 {}
#[doc = "Interrupt Source 40 Priority"]
pub mod prio40;
#[doc = "Interrupt Source 41 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio41](prio41) module"]
pub type PRIO41 = crate::Reg<u8, _PRIO41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO41;
#[doc = "`read()` method returns [prio41::R](prio41::R) reader structure"]
impl crate::Readable for PRIO41 {}
#[doc = "`write(|w| ..)` method takes [prio41::W](prio41::W) writer structure"]
impl crate::Writable for PRIO41 {}
#[doc = "Interrupt Source 41 Priority"]
pub mod prio41;
#[doc = "Interrupt Source 42 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio42](prio42) module"]
pub type PRIO42 = crate::Reg<u8, _PRIO42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO42;
#[doc = "`read()` method returns [prio42::R](prio42::R) reader structure"]
impl crate::Readable for PRIO42 {}
#[doc = "`write(|w| ..)` method takes [prio42::W](prio42::W) writer structure"]
impl crate::Writable for PRIO42 {}
#[doc = "Interrupt Source 42 Priority"]
pub mod prio42;
#[doc = "Interrupt Source 43 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio43](prio43) module"]
pub type PRIO43 = crate::Reg<u8, _PRIO43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO43;
#[doc = "`read()` method returns [prio43::R](prio43::R) reader structure"]
impl crate::Readable for PRIO43 {}
#[doc = "`write(|w| ..)` method takes [prio43::W](prio43::W) writer structure"]
impl crate::Writable for PRIO43 {}
#[doc = "Interrupt Source 43 Priority"]
pub mod prio43;
#[doc = "Interrupt Source 44 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio44](prio44) module"]
pub type PRIO44 = crate::Reg<u8, _PRIO44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO44;
#[doc = "`read()` method returns [prio44::R](prio44::R) reader structure"]
impl crate::Readable for PRIO44 {}
#[doc = "`write(|w| ..)` method takes [prio44::W](prio44::W) writer structure"]
impl crate::Writable for PRIO44 {}
#[doc = "Interrupt Source 44 Priority"]
pub mod prio44;
#[doc = "Interrupt Source 45 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio45](prio45) module"]
pub type PRIO45 = crate::Reg<u8, _PRIO45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO45;
#[doc = "`read()` method returns [prio45::R](prio45::R) reader structure"]
impl crate::Readable for PRIO45 {}
#[doc = "`write(|w| ..)` method takes [prio45::W](prio45::W) writer structure"]
impl crate::Writable for PRIO45 {}
#[doc = "Interrupt Source 45 Priority"]
pub mod prio45;
#[doc = "Interrupt Source 46 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio46](prio46) module"]
pub type PRIO46 = crate::Reg<u8, _PRIO46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO46;
#[doc = "`read()` method returns [prio46::R](prio46::R) reader structure"]
impl crate::Readable for PRIO46 {}
#[doc = "`write(|w| ..)` method takes [prio46::W](prio46::W) writer structure"]
impl crate::Writable for PRIO46 {}
#[doc = "Interrupt Source 46 Priority"]
pub mod prio46;
#[doc = "Interrupt Source 47 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio47](prio47) module"]
pub type PRIO47 = crate::Reg<u8, _PRIO47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO47;
#[doc = "`read()` method returns [prio47::R](prio47::R) reader structure"]
impl crate::Readable for PRIO47 {}
#[doc = "`write(|w| ..)` method takes [prio47::W](prio47::W) writer structure"]
impl crate::Writable for PRIO47 {}
#[doc = "Interrupt Source 47 Priority"]
pub mod prio47;
#[doc = "Interrupt Source 48 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio48](prio48) module"]
pub type PRIO48 = crate::Reg<u8, _PRIO48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO48;
#[doc = "`read()` method returns [prio48::R](prio48::R) reader structure"]
impl crate::Readable for PRIO48 {}
#[doc = "`write(|w| ..)` method takes [prio48::W](prio48::W) writer structure"]
impl crate::Writable for PRIO48 {}
#[doc = "Interrupt Source 48 Priority"]
pub mod prio48;
#[doc = "Interrupt Source 49 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio49](prio49) module"]
pub type PRIO49 = crate::Reg<u8, _PRIO49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO49;
#[doc = "`read()` method returns [prio49::R](prio49::R) reader structure"]
impl crate::Readable for PRIO49 {}
#[doc = "`write(|w| ..)` method takes [prio49::W](prio49::W) writer structure"]
impl crate::Writable for PRIO49 {}
#[doc = "Interrupt Source 49 Priority"]
pub mod prio49;
#[doc = "Interrupt Source 50 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio50](prio50) module"]
pub type PRIO50 = crate::Reg<u8, _PRIO50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO50;
#[doc = "`read()` method returns [prio50::R](prio50::R) reader structure"]
impl crate::Readable for PRIO50 {}
#[doc = "`write(|w| ..)` method takes [prio50::W](prio50::W) writer structure"]
impl crate::Writable for PRIO50 {}
#[doc = "Interrupt Source 50 Priority"]
pub mod prio50;
#[doc = "Interrupt Source 51 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio51](prio51) module"]
pub type PRIO51 = crate::Reg<u8, _PRIO51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO51;
#[doc = "`read()` method returns [prio51::R](prio51::R) reader structure"]
impl crate::Readable for PRIO51 {}
#[doc = "`write(|w| ..)` method takes [prio51::W](prio51::W) writer structure"]
impl crate::Writable for PRIO51 {}
#[doc = "Interrupt Source 51 Priority"]
pub mod prio51;
#[doc = "Interrupt Source 52 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio52](prio52) module"]
pub type PRIO52 = crate::Reg<u8, _PRIO52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO52;
#[doc = "`read()` method returns [prio52::R](prio52::R) reader structure"]
impl crate::Readable for PRIO52 {}
#[doc = "`write(|w| ..)` method takes [prio52::W](prio52::W) writer structure"]
impl crate::Writable for PRIO52 {}
#[doc = "Interrupt Source 52 Priority"]
pub mod prio52;
#[doc = "Interrupt Source 53 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio53](prio53) module"]
pub type PRIO53 = crate::Reg<u8, _PRIO53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO53;
#[doc = "`read()` method returns [prio53::R](prio53::R) reader structure"]
impl crate::Readable for PRIO53 {}
#[doc = "`write(|w| ..)` method takes [prio53::W](prio53::W) writer structure"]
impl crate::Writable for PRIO53 {}
#[doc = "Interrupt Source 53 Priority"]
pub mod prio53;
#[doc = "Interrupt Source 54 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio54](prio54) module"]
pub type PRIO54 = crate::Reg<u8, _PRIO54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO54;
#[doc = "`read()` method returns [prio54::R](prio54::R) reader structure"]
impl crate::Readable for PRIO54 {}
#[doc = "`write(|w| ..)` method takes [prio54::W](prio54::W) writer structure"]
impl crate::Writable for PRIO54 {}
#[doc = "Interrupt Source 54 Priority"]
pub mod prio54;
#[doc = "Interrupt Source 55 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio55](prio55) module"]
pub type PRIO55 = crate::Reg<u8, _PRIO55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO55;
#[doc = "`read()` method returns [prio55::R](prio55::R) reader structure"]
impl crate::Readable for PRIO55 {}
#[doc = "`write(|w| ..)` method takes [prio55::W](prio55::W) writer structure"]
impl crate::Writable for PRIO55 {}
#[doc = "Interrupt Source 55 Priority"]
pub mod prio55;
#[doc = "Interrupt Source 56 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio56](prio56) module"]
pub type PRIO56 = crate::Reg<u8, _PRIO56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO56;
#[doc = "`read()` method returns [prio56::R](prio56::R) reader structure"]
impl crate::Readable for PRIO56 {}
#[doc = "`write(|w| ..)` method takes [prio56::W](prio56::W) writer structure"]
impl crate::Writable for PRIO56 {}
#[doc = "Interrupt Source 56 Priority"]
pub mod prio56;
#[doc = "Interrupt Source 57 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio57](prio57) module"]
pub type PRIO57 = crate::Reg<u8, _PRIO57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO57;
#[doc = "`read()` method returns [prio57::R](prio57::R) reader structure"]
impl crate::Readable for PRIO57 {}
#[doc = "`write(|w| ..)` method takes [prio57::W](prio57::W) writer structure"]
impl crate::Writable for PRIO57 {}
#[doc = "Interrupt Source 57 Priority"]
pub mod prio57;
#[doc = "Interrupt Source 58 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio58](prio58) module"]
pub type PRIO58 = crate::Reg<u8, _PRIO58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO58;
#[doc = "`read()` method returns [prio58::R](prio58::R) reader structure"]
impl crate::Readable for PRIO58 {}
#[doc = "`write(|w| ..)` method takes [prio58::W](prio58::W) writer structure"]
impl crate::Writable for PRIO58 {}
#[doc = "Interrupt Source 58 Priority"]
pub mod prio58;
#[doc = "Interrupt Source 59 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio59](prio59) module"]
pub type PRIO59 = crate::Reg<u8, _PRIO59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO59;
#[doc = "`read()` method returns [prio59::R](prio59::R) reader structure"]
impl crate::Readable for PRIO59 {}
#[doc = "`write(|w| ..)` method takes [prio59::W](prio59::W) writer structure"]
impl crate::Writable for PRIO59 {}
#[doc = "Interrupt Source 59 Priority"]
pub mod prio59;
#[doc = "Interrupt Source 60 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio60](prio60) module"]
pub type PRIO60 = crate::Reg<u8, _PRIO60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO60;
#[doc = "`read()` method returns [prio60::R](prio60::R) reader structure"]
impl crate::Readable for PRIO60 {}
#[doc = "`write(|w| ..)` method takes [prio60::W](prio60::W) writer structure"]
impl crate::Writable for PRIO60 {}
#[doc = "Interrupt Source 60 Priority"]
pub mod prio60;
#[doc = "Interrupt Source 61 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio61](prio61) module"]
pub type PRIO61 = crate::Reg<u8, _PRIO61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO61;
#[doc = "`read()` method returns [prio61::R](prio61::R) reader structure"]
impl crate::Readable for PRIO61 {}
#[doc = "`write(|w| ..)` method takes [prio61::W](prio61::W) writer structure"]
impl crate::Writable for PRIO61 {}
#[doc = "Interrupt Source 61 Priority"]
pub mod prio61;
#[doc = "Interrupt Source 62 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio62](prio62) module"]
pub type PRIO62 = crate::Reg<u8, _PRIO62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIO62;
#[doc = "`read()` method returns [prio62::R](prio62::R) reader structure"]
impl crate::Readable for PRIO62 {}
#[doc = "`write(|w| ..)` method takes [prio62::W](prio62::W) writer structure"]
impl crate::Writable for PRIO62 {}
#[doc = "Interrupt Source 62 Priority"]
pub mod prio62;
#[doc = "Threshold of priority for Target 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold0](threshold0) module"]
pub type THRESHOLD0 = crate::Reg<u8, _THRESHOLD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESHOLD0;
#[doc = "`read()` method returns [threshold0::R](threshold0::R) reader structure"]
impl crate::Readable for THRESHOLD0 {}
#[doc = "`write(|w| ..)` method takes [threshold0::W](threshold0::W) writer structure"]
impl crate::Writable for THRESHOLD0 {}
#[doc = "Threshold of priority for Target 0"]
pub mod threshold0;
#[doc = "Claim interrupt by read, complete interrupt by write for Target 0. Value read/written is interrupt ID. Reading a value of 0 means no pending interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0](cc0) module"]
pub type CC0 = crate::Reg<u8, _CC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0;
#[doc = "`read()` method returns [cc0::R](cc0::R) reader structure"]
impl crate::Readable for CC0 {}
#[doc = "`write(|w| ..)` method takes [cc0::W](cc0::W) writer structure"]
impl crate::Writable for CC0 {}
#[doc = "Claim interrupt by read, complete interrupt by write for Target 0. Value read/written is interrupt ID. Reading a value of 0 means no pending interrupts."]
pub mod cc0;
#[doc = "msip for Hart 0. Write 1 to here asserts software interrupt for Hart msip_o\\[0\\], write 0 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msip0](msip0) module"]
pub type MSIP0 = crate::Reg<u8, _MSIP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIP0;
#[doc = "`read()` method returns [msip0::R](msip0::R) reader structure"]
impl crate::Readable for MSIP0 {}
#[doc = "`write(|w| ..)` method takes [msip0::W](msip0::W) writer structure"]
impl crate::Writable for MSIP0 {}
#[doc = "msip for Hart 0. Write 1 to here asserts software interrupt for Hart msip_o\\[0\\], write 0 to clear."]
pub mod msip0;
