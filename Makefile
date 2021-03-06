# Make rules to simplify the process of building a Rust binary to run
# on the OpenTian earlgrey RISC-V processor.
#
# Building a Rust binary for barebones RISC-V is a two-stage process.
# First, `cargo` and `rustc` compile and link a static library of all
# Rust code. Following this a RISC-V linker must take the Rust library
# and link in a small assembler stub which jumps into the Rust code.
#
# The linker needs two files to do this: a linker script specifying the
# chip's memory layout, and an assembler stub file. Both of these were
# taken nearly verbatim from OpenTitan' sw/device/exts/common. The only
# change was to remove the `GROUP(-lgcc)` line from the linker script,
# since we're only running Rust code the GCC runtime isn't required.
#
# This script assumes you have already install the Rust compiler and
# Cargo build manager. The existing Rust code has been tested using
# `rustup` and Rust 1.42 nightly.
#
# In addition to Rust you need to have a RISC-V 32-bit assembler and
# linker available. The OpenTitan project distributes a prebuilt copy
# in a Docker container; alternatively this includes rules to build a
# copy of the GNU binutils `as` and `ld`. Run `make toolchain`, this
# will download the latest (2.33.1) version of binutils and compile
# `riscv32-unknown-elf-ld` and `riscv32-unknown-elf-as` automatically.
# (You will of course need to first install a working C toolchain for
# your host system.)

# These are the packages that will be built; one .elf for each.
EXAMPLES ?= \
	earlgrey-uart \

# Target selection for Rust and binutils (as & ld).
#
# TODO: Some of this duplicates information in .cargo/config. Is it
# possible to unravel this?
BINUTILS_TARGET ?= riscv32-unknown-elf-
RUST_TARGET     ?= riscv32imc-unknown-none-elf
RUST_BUILD      ?= release
SVD2RUST        ?= svd2rust
CRATE_LIB_DIR   := target/$(RUST_TARGET)/$(RUST_BUILD)

# Path to the SVD file defining the registers. Used to generate the
# `earlgrey-registers` crate.
OPENTITAN_DIR   ?= ../opentitan
EARLGREY_SVD    := hw/top_earlgrey/data/top_earlgrey.svd

# Configure binutils for building an Ibex binary.
#
# We avoid using the make AS and LD variables, or their associated
# default rules to avoid stepping on the toes of the toolchain build.
RISCV_AS    ?= $(BINUTILS_TARGET)as
RISCV_LD    ?= $(BINUTILS_TARGET)ld
RISCV_STRIP ?= $(BINUTILS_TARGET)strip
RISCV_ASFLAGS    ?= -march=rv32imc -mabi=ilp32
RISCV_LDFLAGS    ?= -static --no-dynamic-linker -nostdlib --gc-sections
RISCV_STRIPFLAGS ?= -sD

# `svd2rust` emits some lints which are no longer supported in rustc.
# The converted Rust lib.rs will be pruned of all matching allow/deny
# lints:
SVD2RUST_TRIM_LINTS ?= \
	legacy_directory_ownership \
	plugin_as_library \
	safe_extern_statics \
	unions_with_drop_fields \

# Set the default target: .elf files for all examples.
#
# Cargo converts hyphens in the crate name to underscores in the output
# archive name.
all: $(subst -,_,$(EXAMPLES:%=%.elf))

# Custom assembler rules; make's default rules don't support comments
# in the assembly file as used by OpenTitan. Since we're avoiding use
# of GCC and directly invoking the linker this must be done manually.
%.o: %.S; set -o pipefail; $(CPP) $< | $(RISCV_AS) $(RISCV_ASFLAGS) -o $@

# Link rules to take a Rust static library and create a RISC-V binary.
#
# This rule accepts any number of static libraries, linker scripts or
# object files:
#   * linker scripts ending in `.ld` are passed with `-T`
#   * all non-linker, non-static-libraries are passed unchanged
#   * directories for all static-libraries are passed with `-L`
#   * link names of all static-libraries are passed with `-l`
# These values are determined automatically from the dependency list.
# Note that the suffix of linker scripts and object files matter (as
# is standard for make rules).
%.elf: $(CRATE_LIB_DIR)/lib%.a flash_link.ld flash_crt.o
	$(RISCV_LD) $(RISCV_LDFLAGS) -o $@ \
		$(addprefix -T,$(filter %.ld,$^)) \
		$(filter-out %.ld %.a,$^) \
		$(addprefix -L,$(dir $(filter %.a,$^))) \
		$(patsubst lib%.a,-l%,$(filter lib%.a,$(^F)))

# Strip the elf binary. This drops the binary size by a few orders of
# magnitude, and brings the total size in-line with C binaries.
%.stripped: %.elf; $(RISCV_STRIP) $(RISCV_STRIPFLAGS) $< -o $@

# Tie the make build into Cargo's. We do our best to recreate the full
# dependency tree on the Rust source, but this might be better as a
# .PHONY target to force a rebuild every time.
#
# We search for all Cargo configuration and Rust source files to set as
# prerequisites. This is likely overzealous, but it's better to err on
# the side of caution.
#
# One hiccup of this is that while Cargo will rebuild the library if
# any Rust source file's timestamp changes, it won't run a rebuild if
# the Cargo.toml file's timestamp changes. To avoid the degenerate case
# where make thinks a rebuild is necessary, but Cargo doesn't we `touch`
# the library if Cargo succeeds, thus ensuring the timestamp changes.
#
# TODO: this builds all targets on each execution. Using --lib doesn't
# work, find a solution to build only the single staticlib needed. For
# now this doesn't matter, since there's only one target anyway.
$(CRATE_LIB_DIR)/lib%.a: $(shell find . -name '*.rs' -or -name Cargo.toml)
	cargo build --$(RUST_BUILD) && touch $@

# Make includes a `join` function. It lies.
#
# `car` and `cdr` mimic the LISP funtions of the same name. In a more
# sane world this would be something like:
#   strjoin(sep, words) = words[0] + (if words[1:] != "" then: sep + strjoin(sep, words[1:]))
car = $(firstword $1)
cdr = $(wordlist 2,$(words $1),$1)
strjoin = $(call car,$2)$(if $(call cdr,$2),$1$(call strjoin,$1,$(call cdr,$2)))

# A cheap rule to update the earlgrey-registers crate when the upstream
# SVD file changes.
#
# We run `sed` over the resulting file to trim any `allow`/`deny` lints
# which are out-of-date. Without this the generated code emits warnings
# for the unknown lints.
earlgrey-registers/lib.rs: LINT_MATCH := ($(call strjoin,|,$(SVD2RUST_TRIM_LINTS)))
earlgrey-registers/lib.rs: LINT_REGEX := ^\#!\[(allow|deny)\($(LINT_MATCH)\)]$$
earlgrey-registers/lib.rs: $(realpath $(OPENTITAN_DIR)/$(EARLGREY_SVD))
	@# this incantation is ugly enough to hide...
	@echo $(SVD2RUST) --target riscv -i $< -o $@
	@(cd $(@D) && $(SVD2RUST) --target riscv -i $< && rustfmt $(@F)) && \
		sed -i "" -Ee '/$(LINT_REGEX)/d' $@

# make deletes the compiled Rust library archive unless told otherwise.
.SECONDARY: flash_crt.o \
	$(addprefix $(CRATE_LIB_DIR)/,$(subst -,_,$(EXAMPLES:%=lib%.a)))

clean: ; $(RM) flash_crt.o $(subst -,_,$(EXAMPLES:%=%.elf)); cargo clean
.PHONY: all clean toolchain

# Instructions to build a local copy of the binutils `as` and `ld` tools
# to allow linking the Rust libraries into a full binary.
toolchain: $(addprefix $(BINUTILS_TARGET),as ld strip)

BINUTILS_VERSION ?= 2.33.1
PARALLEL_BUILD   ?= $(or $(shell (nproc || sysctl -n hw.ncpu) 2>/dev/null),2)
BINUTILS_DIR     := binutils-$(BINUTILS_VERSION)

$(BINUTILS_TARGET)as:    $(BINUTILS_DIR)/build/gas/as-new;         cp $< $@
$(BINUTILS_TARGET)ld:    $(BINUTILS_DIR)/build/ld/ld-new;          cp $< $@
$(BINUTILS_TARGET)strip: $(BINUTILS_DIR)/build/binutils/strip-new; cp $< $@
$(BINUTILS_DIR)/build/%-new: $(BINUTILS_DIR)/build/Makefile
	$(MAKE) -j$(PARALLEL_BUILD) -C $(<D) all-$(notdir $(*D))
$(BINUTILS_DIR)/build/Makefile: $(BINUTILS_DIR)/configure | $(BINUTILS_DIR)/build
	(cd $(@D) && ../$(<F) \
		--target $(BINUTILS_TARGET:%-=%) \
		--disable-gold \
		--enable-ld \
		--disable-libquadmath \
		--disable-libstdcxx \
		--disable-isl-version-check \
		--enable-lto \
		--disable-host-shared \
	)
$(BINUTILS_DIR)/build: ; mkdir -p $@
$(BINUTILS_DIR)/configure: | binutils-$(BINUTILS_VERSION).tar.xz; tar -xJf $|
binutils-$(BINUTILS_VERSION).tar.xz:
	curl https://ftp.gnu.org/gnu/binutils/$@ -o $@
