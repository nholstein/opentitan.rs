extern crate earlgrey_registers;

use std::convert::TryFrom;

#[cfg(ibex = "verilator")]
const CLOCK_FREQ_HZ: u32 = 50_000;

#[cfg(not(ibex = "verilator"))]
const CLOCK_FREQ_HZ: u32 = 50_000_000;

#[cfg(ibex = "verilator")]
const BAUD_RATE: u32 = 9600;

#[cfg(not(ibex = "verilator"))]
const BAUD_RATE: u32 = 230400;

fn start() {
	let peripherals = earlgrey_registers::Peripherals::take().unwrap();

	uart_init(&peripherals.UART, BAUD_RATE, true, true).unwrap();
}

fn uart_init(uart: &earlgrey_registers::UART, baud: u32, enable_parity: bool, odd_parity: bool) -> Result<(), std::num::TryFromIntError>{
	let nco = ((BAUD_RATE as u64) << 20) / (CLOCK_FREQ_HZ as u64);
	let nco = u16::try_from(nco)?;

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

	Ok(())
}
