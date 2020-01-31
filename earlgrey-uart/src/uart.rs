extern crate earlgrey_registers;

//use std::convert::TryFrom;

#[cfg(ibex = "verilator")]
const CLOCK_FREQ_HZ: u32 = 500_000;

#[cfg(not(ibex = "verilator"))]
const CLOCK_FREQ_HZ: u32 = 50_000_000;

#[cfg(ibex = "verilator")]
const BAUD_RATE: u32 = 9600;

#[cfg(not(ibex = "verilator"))]
const BAUD_RATE: u32 = 230400;

pub fn uart_echo() {
	let peripherals = earlgrey_registers::Peripherals::take().unwrap();

	let uart = &peripherals.UART;
	uart_init(uart, BAUD_RATE, true, true);
	uart_write(uart, &"Hello, Rust");

	loop {
		let c = uart_getc(uart);
		uart_putc(uart, c);
	}
}

fn uart_init(uart: &earlgrey_registers::UART, baud: u32, enable_parity: bool, odd_parity: bool) {
	let nco = ((BAUD_RATE as u64) << 20) / (CLOCK_FREQ_HZ as u64);
	let nco = nco as u16;

	uart.ctrl.write(|w| unsafe {
		w.nco().bits(nco);
		w.tx().set_bit();
		w.rx().set_bit();

		if enable_parity {
			w.parity_en().set_bit();
		}

		if odd_parity {
			w.parity_odd().set_bit();
		}

		w
	});

	uart.fifo_ctrl.write(|w| {
		w.rxrst().set_bit();
		w.txrst().set_bit()
	});

	uart.intr_enable.write(|w| w);
}

fn uart_putc(uart: &earlgrey_registers::UART, c: char) {
	while uart.status.read().txfull().bit() {
	}

	let u = c as u8;
	uart.wdata.write(|w| unsafe { w.bits(u) })
}

fn uart_write(uart: &earlgrey_registers::UART, message: &str) {
	for b in message.chars() {
		uart_putc(uart, b);
	}
}

fn uart_getc(uart: &earlgrey_registers::UART) -> char {
	while uart.status.read().rxempty().bit() {
	}

	uart.rdata.read().bits() as char
}
