# Exploring Cargo cross-builds

These are notes containing the highlights of experimentation into ways to
perform bare-metal linking and customization of the Cargo build process.
While Cargo is a powerful dependency manager and offers numerous
features, its ability to perform customized linking as needed to run code
on bare-meral OpenTitan `earlgrey` processors is intentionally limited.

Since there is no standardized way to link these binaries, multiple
alternative exist, with tradeoffs and advantages between them. Of the
ones I investigated, the only clear-cut advantage I discovered was that
use of `global_asm!()` from within Rust was consistently simpler than
using a separate assembly file.

Additionally, while I believe it is possible for Cargo to complete the
full build of an ELF executable for any given target, this will likely
need to be combined with external build scripts run by `make`, `meson`,
or similar to perform additional steps such as stripping debug
information or conversion to raw flash images or Verilog.

## Need for an external linker

Rust currently calls out to the system linker to perform this build step.
For a RISC-V bare metal system this means a third-party linker executable
is required to complete the build. While the LLVM project does provide a
linker, in practice this means either binutils' `riscv32-unknown-elf-ld`
needs to be installed, or both `riscv32-unknown-elf-ld` and
`riscv32-unknown-elf-gcc`.

These must be installed separately from Rust, and are not available
through Rust package management such as `rustup`.

In terms of complexity of the install, a GCC toolchain is far more
complicated to build and install than just binutils. In fact, all that is
strictly required is `ld`. For all practical purposes howerver, the full
binutils suite with `strip`, `objdump`, `readelf`, ...etc is required.

However, GCC isn't required. Particularly when combined with use of
`global_asm!()` to define the CRT it provides no additional features over
`ld`, and only increases the toolcahin installation cost.

## Customizing the linker

Linking a bare-metal binary requires two items not provided by `cargo`:

 * the linker script defining the system memory layout

   This is passed with the `-T` option to GCC/ld.

 * the CRT and startup routines in assembly code

   This must be assembled by either GCC or `as`. This can be a standalone
   file or a macro using [`global_asm!()`](https://doc.rust-lang.org/beta/unstable-book/library-features/global-asm.html).
   The second option is transparent to the linker and elminates the need
   to use either GCC or `as`; the CRT is included automatically in the
   compiled Rust objects passed to the linker.

   A standalone assembly file likely needs preprocessing to strip out
   comments. `cc -E` does this, but introduces one more dependency on
   the developer's machine. In contrast, inline assembly in Rust may
   include comments.

   Additionally, a standlone assembly file must be build outside of the
   normal Cargo flow. Whether this is prebuild by `make`, compiled by
   `build.rs`, or included in the linker step, this introduces additional
   complexity and dependency resolution. Use of `global_asm!()` defers
   this work to Cargo's standard depedency tree.

## Running strip/objdump

This step could _theorectically_ be mixed into the customized linker
command, but this is probably better left as an external step by `make`
or `meson`.

## Dependencies and integrating into a build system

Rustc generates dependency files during its compilation step. These are
in the `.d` format used by GCC to record its `#include` dependencies, and
is essentially a pared-down set of `make` dependencies. This format is
understood by some additional tools such as `ninja`. This could allow an
external build manager to invoke `cargo` only when a build file has
changed.

Unfortunately, I only saw this generated a on per-crate basis, and did
not include cross-crate dependencies. This means while it might be
possible to know which Rust source files are used, there's no means to
tell what then requires are rebuild.

In practice, this might not matter. It might be sufficient to either
call `cargo` when any Rust file changes, or simply on every build.

## Places to customize a Cargo build

There are three primary places to inject custom commands into the Cargo
build:

 * A [`build.rs` build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts)
   placed within any crate's directory alongside `Cargo.toml`. Compiled
   and run at the start of the build process, before any code for the
   target is built. Can run any executable, script, or Rust code making
   it highly flexible.

   This is well supported by the Cargo team and used by many crates.

   Can read anything from the repo, i.e. extracting a Git version string.
   Provided an extensive amount of information regarding the build as
   environment variables. Critically, this includes the linker name in
   `$RUSTC_LINKER` and the target in `$TARGET`.

   Cargo does not check for the existence of the linker prior to invoking
   the build script. This means that the build script may create the
   linker itself.

   Can emit directives back to Cargo to control the build process. For
   the purposed of customizing the link process this is actually fairly
   limited. Critically, while it is possible to specify custom link (`-l`)
   libraries and directories (`-L'`), these are the only options passed
   through to the linker, other options are filtered.

 * The Cargo configuration allows specifying a custom linker command.
   Only a single string is allowed, and it is interpreted as the name of
   the command to execute. This means it is not possible to specify any
   arguments to this command.

   This is well-supported by Cargo.

   Paths relative to the top-level build directory may be specified by
   specifying a path containing a `/`. If the path is not relative the
   linker must be found in the user's path.

   The linker is given less information about its environment than is
   passed to `build.rs`. Crucially, it is _not_ passed the build target
   triple as is given to `build.rs` in `$TARGET`. This means there is no
   means for a single, generic, linker script to customize its behavior
   dependent upon the current build.

   (It is passed a directory path where outputs should be written. It is
   possible to parse this and attempt to extract this information, but I
   would consider this an unreliable heuristic method.)

 * A customized Rustc target configuration may be specified using a JSON
   file. This provides the most flexibility, but it relies upon internal
   features of Rust and is only available on nightly (?).

   Documentation is the most sparse. Determining the full set of values
   which may be specified, and their datatypes, requires reading the
   [`rustc` source and doc.rs](https://doc.rust-lang.org/1.1.0/rustc_back/target/struct.Target.html).

   Additionally, using a custom target requires rebuilding the Rust core
   libraries for the target. This is simplified by `cargo-xbuild`
   (formerly `xargo`), but this build was not reliable on my system, and
   I could not get it to work consistently. (OSX 10.15, cargo
   1.41.0-nightly, rustc 1.42.0-nightly, cargo-xbuild 0.5.21)

   The benefit is that this side provides the most flexibility, by far.
   Numerous detailed compilation flags can be specified to fully tweak
   system behavior--some examples are that ASLR can be toggled, and a
   custom suffix defined for build outputs. (`.elf` as currently used by
   OpenTitan.)

   In addition to specifying a linker executable this allows specifying
   custom arguments to the linker as well. This alleviates the need for a
   separate to script to tweaks the arguments as is necesary above.

     * The failure may be related to the `riscv` crate not recognizing a
       custom target triplet. I could get the build to succeed if I
       reused the `riscv32imc-unknown-none-elf` triplet, but failed when
       instead using `riscv32imc-unknown-none-elf`, even if the config
       JSON was identical.

## Cargo xbuild vs. rustup target install

The two options listed above require different Rust installation steps.
Fortunately, these are not exclusive, and a single development system and
Rust installation can run both simultaneously without conflict.

The standard is simply using `rustup` to install. This is the required
option when using `.cargo/config` to make the target's core libraries
available:

```
rustup target install riscv32imc-unknown-none-elf
```

When using a custom target JSON you'll need to provide the pre-compiled
core libraries outside of `rustup`. The `cargo-xbuild` project greatly
simplifies this process:

```
rustup component add rust-src
cargo install cargo-xbuild
```

The disadvantage of this option is that developers need to remeber to
replace `cargo build` with `cargo xbuild`.

## Methodoloy

There are good sources of information available online for building
bare-metal systems in Rust.

Nontheless, gathering this data required a fair amount of experimentation.
In addition to the links included above I frequently referred to some of
the following:

 * The Cargo [configuration reference](https://doc.rust-lang.org/cargo/reference/config.html).

 * The Cargo [manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html).

 * The Cargo [build scripts reference](https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts).

 * The [cargo-xbuild documentation](https://github.com/rust-osdev/cargo-xbuild).

Additionally, I used some simple scripts to help with testing.

It is possible to view the output of a Cargo build script by telling
Cargo to use very verbose output with `cargo build -vv`. It's easy to
then print the command line and environment:

```
use std::env;

fn main() {
	println!("exe: {}", env::current_exe().unwrap().to_str().unwrap());
	println!("dir: {}", env::current_dir().unwrap().to_str().unwrap());

	println!("args:");
	for argument in env::args() {
		println!("\t{}", argument);
	}

	println!("env:");
	for (key, value) in env::vars() {
		println!("\t{}: {}", key, value);
	}
}
```

It isn't as easy to glean data from the linking stage. Cargo and `rustc`
seemed to hide its output, no matter how many `-v`s were given. To avoid
this I wrote this data to files:

```
#!/bin/sh

env > riscv32-unknown-elf-ld.sh.env
echo "$0" "$@" > riscv32-unknown-elf-ld.sh.args

exec riscv32-unknown-elf-ld \
	-static \
	--no-dynamic-linker \
	-nostdlib \
	--gc-sections \
	-Tflash_link.ld \
	"$@"
```
