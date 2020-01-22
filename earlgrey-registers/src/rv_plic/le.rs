#[doc = "Interrupt Source mode. 0: Level, 1: Edge-triggered\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le0](le0) module"]
pub type LE0 = crate::Reg<u32, _LE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE0;
#[doc = "`read()` method returns [le0::R](le0::R) reader structure"]
impl crate::Readable for LE0 {}
#[doc = "`write(|w| ..)` method takes [le0::W](le0::W) writer structure"]
impl crate::Writable for LE0 {}
#[doc = "Interrupt Source mode. 0: Level, 1: Edge-triggered"]
pub mod le0;
#[doc = "Interrupt Source mode. 0: Level, 1: Edge-triggered\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le1](le1) module"]
pub type LE1 = crate::Reg<u32, _LE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE1;
#[doc = "`read()` method returns [le1::R](le1::R) reader structure"]
impl crate::Readable for LE1 {}
#[doc = "`write(|w| ..)` method takes [le1::W](le1::W) writer structure"]
impl crate::Writable for LE1 {}
#[doc = "Interrupt Source mode. 0: Level, 1: Edge-triggered"]
pub mod le1;
