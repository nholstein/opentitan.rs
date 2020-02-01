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
	let mut uart = UART::new(&peripherals, UART::DEFAULT_BAUD).unwrap();

	// Ensure we can access other modules within peripherals
	let mut _aes = &peripherals.AES;

	uart.write(&"Hello, Rust\r\n");

	loop {
		let byte = uart.get_byte();
		uart.put_byte(byte);
	}
}
