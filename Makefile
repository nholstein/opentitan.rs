EXAMPLES = \
	earlgrey-uart \

# Target selection for Rust and binutils (as & ld)
CROSS_COMPILE ?= riscv32-unknown-elf-
RUST_TARGET   ?= riscv32imac-unknown-none-elf
RUST_BUILD    ?= release

# Override make's defaults. Note that we use the host's default C
# preprocessor to filter comments from flash_crt.S; this isn't platform
# specific.
AS = $(CROSS_COMPILE)as
LD = $(CROSS_COMPILE)ld

# Configure binutils for building an Ibex binary.
ASFLAGS += -march=rv32imac -mabi=ilp32
LDFLAGS += -static --no-dynamic-linker -nostdlib

# Set the default target: .elf files for all examples.
#
# Cargo converts hyphens in the crate name to underscores in the output
# archive name.
all: $(subst -,_,$(EXAMPLES:%=%.elf))

# Custom assembler rules; make's default rules don't support comments
# in the assembly file as used by OpenTitan. Since we're avoiding use
# of GCC and directly invoking the linker this must be done manually.
%.o: %.S; set -o pipefail; $(CPP) $< | $(AS) $(ASFLAGS) -o $@

# Link rules to take a Rust static library and create a RISC-V binary.
CRATE_LIB_DIR := target/$(RUST_TARGET)/$(RUST_BUILD)
%.elf: $(CRATE_LIB_DIR)/lib%.a flash_link.ld flash_crt.o
	$(LD) $(LDFLAGS) -o $@ \
		$(addprefix -T,$(filter %.ld,$^)) \
		$(filter-out %.ld %.a,$^) \
		$(addprefix -L,$(dir $(filter %.a,$^))) \
		$(patsubst lib%.a,-l%,$(filter lib%.a,$(^F)))

# Tie the make build into Cargo's. We do our best to recreate the full
# dependency tree on the Rust source, but this might be better as a
# .PHONY target to force a rebuild every time.
#
# TODO: this builds all targets on each execution. Using --lib doesn't
# work, find a solution to only build the single staticlib needed.
$(CRATE_LIB_DIR)/lib%.a: Cargo.toml \
		$(shell find earlgrey-* -name '*.rs' -name Cargo.toml)
	cargo build --target $(RUST_TARGET) --$(RUST_BUILD)

# make deletes the compiled Rust library archive unless told otherwise.
.SECONDARY: flash_crt.o \
	$(addprefix $(CRATE_LIB_DIR)/,$(subst -,_,$(EXAMPLES:%=lib%.a)))

clean: ; $(RM) flash_crt.o $(subst -,_,$(EXAMPLES:%=%.elf)); cargo clean
.PHONY: all clean
