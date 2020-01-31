CC          = riscv32-unknown-elf-gcc
CFLAGS      = -static -ffreestanding -nostdlib
TARGET     ?= riscv32imac-unknown-none-elf
BUILD_TYPE ?= debug

earlgrey_uart.elf:

%.elf: target/$(TARGET)/$(BUILD_TYPE)/lib%.a
	$(CC) $(CFLAGS) -Tflash_link.ld flash_crt.S -L$(<D) -l$* -o $@

target/$(TARGET)/$(BUILD_TYPE)/lib%.a: Cargo.toml \
		$(shell find earlgrey-* -name '*.rs' -name Cargo.toml)
	cargo build --target $(TARGET)
