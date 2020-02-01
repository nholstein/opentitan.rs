use cast;
use core::fmt;

extern crate earlgrey_registers;

#[cfg(feature = "verilator")]
const CLOCK_FREQ_HZ: u32 = 500_000;

#[cfg(not(feature = "verilator"))]
const CLOCK_FREQ_HZ: u32 = 50_000_000;

#[cfg(feature = "verilator")]
const BAUD_RATE: u32 = 9600;

#[cfg(not(feature = "verilator"))]
const BAUD_RATE: u32 = 230400;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
	// Conversion error in baud rate
	InvalidBaudRate(cast::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Error::InvalidBaudRate(_) => "unsupported baud rate",
		})
	}
}

impl From<cast::Error> for Error {
	fn from(err: cast::Error) -> Error {
		Error::InvalidBaudRate(err)
	}
}

pub struct UART<'a>(&'a earlgrey_registers::UART);

impl UART<'_> {
	pub const DEFAULT_BAUD: u32 = BAUD_RATE;

	pub fn new<'a>(peripherals: &'a earlgrey_registers::Peripherals, baud: u32) -> Result<UART, Error> {
		let uart = UART(&peripherals.UART);
		uart.init(baud, false, false)?;
		Ok(uart)
	}

	fn init(&self, baud: u32, enable_parity: bool, odd_parity: bool) -> Result<(), Error> {
		let nco = ((baud as u64) << 20) / (CLOCK_FREQ_HZ as u64);
		// This would typically use TryFrom but it isn't available
		// in a nostd crate.
		let nco = cast::u16(nco)?;

		self.0.ctrl.write(|w| {
			unsafe { w.nco().bits(nco); }

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

		self.0.fifo_ctrl.write(|w| {
			w.rxrst().set_bit();
			w.txrst().set_bit()
		});

		self.0.intr_enable.write(|w| w);
		Ok(())
	}

	pub fn put_byte(&self, byte: u8) {
		while self.0.status.read().txfull().bit() {
		}

		self.0.wdata.write(|w| unsafe { w.bits(byte) })
	}

	pub fn get_byte(&self) -> u8 {
		while self.0.status.read().rxempty().bit() {
		}

		self.0.rdata.read().bits()
	}

	pub fn write(&self, message: &str) {
		for byte in message.bytes() {
			self.put_byte(byte);
		}
	}
}
