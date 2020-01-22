#![doc = "Peripheral access API for EARLGREY microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 0;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "uart"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "uart"]
pub mod uart;
#[doc = "gpio"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "gpio"]
pub mod gpio;
#[doc = "spi_device"]
pub struct SPI_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI_DEVICE {}
impl SPI_DEVICE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi_device::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for SPI_DEVICE {
    type Target = spi_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI_DEVICE::ptr() }
    }
}
#[doc = "spi_device"]
pub mod spi_device;
#[doc = "flash_ctrl"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "flash_ctrl"]
pub mod flash_ctrl;
#[doc = "rv_timer"]
pub struct RV_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RV_TIMER {}
impl RV_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rv_timer::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for RV_TIMER {
    type Target = rv_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RV_TIMER::ptr() }
    }
}
#[doc = "rv_timer"]
pub mod rv_timer;
#[doc = "aes"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4011_0000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "aes"]
pub mod aes;
#[doc = "hmac"]
pub struct HMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMAC {}
impl HMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmac::RegisterBlock {
        0x4012_0000 as *const _
    }
}
impl Deref for HMAC {
    type Target = hmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMAC::ptr() }
    }
}
#[doc = "hmac"]
pub mod hmac;
#[doc = "rv_plic"]
pub struct RV_PLIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RV_PLIC {}
impl RV_PLIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rv_plic::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for RV_PLIC {
    type Target = rv_plic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RV_PLIC::ptr() }
    }
}
#[doc = "rv_plic"]
pub mod rv_plic;
#[doc = "pinmux"]
pub struct PINMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINMUX {}
impl PINMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pinmux::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for PINMUX {
    type Target = pinmux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINMUX::ptr() }
    }
}
#[doc = "pinmux"]
pub mod pinmux;
#[doc = "alert_handler"]
pub struct ALERT_HANDLER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ALERT_HANDLER {}
impl ALERT_HANDLER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const alert_handler::RegisterBlock {
        0x4013_0000 as *const _
    }
}
impl Deref for ALERT_HANDLER {
    type Target = alert_handler::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ALERT_HANDLER::ptr() }
    }
}
#[doc = "alert_handler"]
pub mod alert_handler;
#[doc = "nmi_gen"]
pub struct NMI_GEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NMI_GEN {}
impl NMI_GEN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nmi_gen::RegisterBlock {
        0x4014_0000 as *const _
    }
}
impl Deref for NMI_GEN {
    type Target = nmi_gen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NMI_GEN::ptr() }
    }
}
#[doc = "nmi_gen"]
pub mod nmi_gen;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SPI_DEVICE"]
    pub SPI_DEVICE: SPI_DEVICE,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "RV_TIMER"]
    pub RV_TIMER: RV_TIMER,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "HMAC"]
    pub HMAC: HMAC,
    #[doc = "RV_PLIC"]
    pub RV_PLIC: RV_PLIC,
    #[doc = "PINMUX"]
    pub PINMUX: PINMUX,
    #[doc = "ALERT_HANDLER"]
    pub ALERT_HANDLER: ALERT_HANDLER,
    #[doc = "NMI_GEN"]
    pub NMI_GEN: NMI_GEN,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            UART: UART {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SPI_DEVICE: SPI_DEVICE {
                _marker: PhantomData,
            },
            FLASH_CTRL: FLASH_CTRL {
                _marker: PhantomData,
            },
            RV_TIMER: RV_TIMER {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            HMAC: HMAC {
                _marker: PhantomData,
            },
            RV_PLIC: RV_PLIC {
                _marker: PhantomData,
            },
            PINMUX: PINMUX {
                _marker: PhantomData,
            },
            ALERT_HANDLER: ALERT_HANDLER {
                _marker: PhantomData,
            },
            NMI_GEN: NMI_GEN {
                _marker: PhantomData,
            },
        }
    }
}
