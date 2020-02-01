#![no_std]
#![feature(lang_items)]
#![feature(asm)]

pub mod uart;
use uart::UART;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
	// Writing to the UART first requires a static reference to the
	// earlgrey_registers::Peripherals object, and this requires
	// locking that is out-of-scope for a proof-of-concept.
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
fn main() -> ! {
	let peripherals = earlgrey_registers::Peripherals::take().unwrap();
	let uart = UART::new(&peripherals, UART::DEFAULT_BAUD).unwrap();

	// Ensure we can access other modules within peripherals
	let _aes = &peripherals.AES;

	uart.write(&"Hello, Rust");

	loop {
		uart.putc(uart.getc());
	}
}
