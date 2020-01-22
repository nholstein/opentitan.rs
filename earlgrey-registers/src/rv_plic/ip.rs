#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip0](ip0) module"]
pub type IP0 = crate::Reg<u32, _IP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP0;
#[doc = "`read()` method returns [ip0::R](ip0::R) reader structure"]
impl crate::Readable for IP0 {}
#[doc = "Interrupt Pending"]
pub mod ip0;
#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip1](ip1) module"]
pub type IP1 = crate::Reg<u32, _IP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP1;
#[doc = "`read()` method returns [ip1::R](ip1::R) reader structure"]
impl crate::Readable for IP1 {}
#[doc = "Interrupt Pending"]
pub mod ip1;
