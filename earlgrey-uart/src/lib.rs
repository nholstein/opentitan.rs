#![no_std]
#![feature(lang_items)]
#![feature(asm)]

pub mod uart;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi"::::"volatile");
		}
	}
}

#[no_mangle]
extern "C"
fn main() {
	uart::uart_echo();
}
