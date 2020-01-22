#[doc = "Reader of register ADDR"]
pub type R = crate::R<u32, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u32, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start address of a flash transaction. Software should supply the full byte address. The flash controller will then truncate the address as needed. For read operations, the flash controller will truncate to the closest, lower word aligned address. For example, if 0x13 is supplied, the controller will perform a read at address 0x10. Program operations behave similarly, the controller does not have read modified write support. For page erases, the controller will truncate to the closest lower page aligned address. Similarly for bank erases, the controller will truncate to the closest lower bank aligned address."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of a flash transaction. Software should supply the full byte address. The flash controller will then truncate the address as needed. For read operations, the flash controller will truncate to the closest, lower word aligned address. For example, if 0x13 is supplied, the controller will perform a read at address 0x10. Program operations behave similarly, the controller does not have read modified write support. For page erases, the controller will truncate to the closest lower page aligned address. Similarly for bank erases, the controller will truncate to the closest lower bank aligned address."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
