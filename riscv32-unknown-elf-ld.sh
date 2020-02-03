#!/bin/sh
exec riscv32-unknown-elf-ld \
	-static \
	--no-dynamic-linker \
	-nostdlib \
	--gc-sections \
	-Tflash_link.ld \
	"$@"
