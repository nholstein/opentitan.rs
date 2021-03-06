# Rust on OpenTitan

This repository is a proof-of-concept to run Rust code on the OpenTitan "earlgrey" chip and its RISC-V Ibex core using register definitions generated by `svd2rust`.
It is neither pretty code nor finished. YMMV.

To demonstrate running code this implements a trivial "Hello, Rust" over the earlgrey UART.

## License

Most makefiles and Rust code are Copyright 2020 Nathan Holstein and released under the Apache 2.0 license.

The Rust `nostd` code was written while closely following the [RISC-V OS using Rust](http://osblog.stephenmarz.com/ch0.html) blog, licensed under the MIT license.

Linker scripts and assembly routine were directly lifted from the OpenTitan project, and Copyright 2020 lowRISC contributors.
The `earlgrey-registers` crate consists of code generated by `svd2rust` from HJSON/SVD files also from OpenTitan.

## Prerequisites

In addition to the developer setup described in the OpenTitan docs, you'll need at least the following tools:

 * make, xz

 * rust & cargo; I've tested only on 1.42-nightly

 * `svd2rust` and `rustfmt` installed via cargo

 * either:

     * a development toolchain for your host machine, or

     * a prebuilt riscv32-unknown-elf binutils+GCC toolchain

## The build process

This uses the SVD file generated by the build process from [the generate-svd branch](https://github.com/nholstein/opentitan/tree/generate-svd).
It should be possible to checkout that branch, and with Python >= 3.5 run `pip3 install -r python-requirements.txt && make -C hw top_earlgrey/data/top_earlgrey.svd`.

With the SVD generated, it should be possible to use `svd2rust` to generate a crate with all the register definitions.
If you checkout this repository and `opentitan` in the same directory the make build should automatically pick up the latest version.
For ease of use, the latest code is checked into this repository in the `earlgrey-registers` crate.

With this, rust, cargo, and the toolchain in place you should be able to simply run `make`.
With any luck this produces `earlgrey_uart.elf` in the top of the repo.

### Running the simulation

Again, I recommend following OpenTitan's [getting started documentation](https://docs.opentitan.org/doc/ug/getting_started_verilator/).
If successful, after running their `hello_world` you should be left with (among other things) a `uart.log`:

```
root@2c90fb446a5f:/opentitan# cat uart0.log
Version:    opentitan-snapshot-20191101-1-415-g1882bad
Build Date: 2020-01-06, 23:08:25
INFO: Boot ROM initialisation has completed, jump into flash!
Hello World! Jan  6 2020 23:16:19
Watch the LEDs!
Try out the switches on the board
or type anything into the console window.
The LEDs show the ASCII code of the last character.
```

You're now ready to run the Rust P.O.C. This should entail nothing more than swapping out `hello_world.elf` for `earlgrey_uart.elf` from the previous command:

```
root@2c90fb446a5f:/opentitan# build/lowrisc_systems_top_earlgrey_verilator_0.1/sim-verilator/Vtop_earlgrey_verilator \
		--meminit=rom,build-bin/sw/device/sim-verilator/boot_rom/boot_rom.elf \
		--meminit=flash,../opentitan.rs/earlgrey_uart.elf
```

And again, check `uart.log` for results:

```
root@2c90fb446a5f:/opentitan# cat uart0.log
Version:    opentitan-snapshot-20191101-1-415-g1882bad
Build Date: 2020-01-06, 23:08:25
INFO: Boot ROM initialisation has completed, jump into flash!
Hello, Rust
```

(A note for anyone testing on an FPGA: I've only run this code in Verilator, at the very least you'll need to change the default "verilator" feature in Cargo to get this running.)

### Building the RISC-V toolchain (optional)

If you don't have a risvc32 toolchain kicking around, the makefile includes a rule to download binutils-2.33.1 and compile the bare minimum `riscv32-unknown-elf-ld` and `riscv32-unknown-elf-as`.
Running `make toolchain` shouldn't take more than a few minutes with a decent connection and development setup.
I've tested this only on OSX Catalina, but binutils should have a stable build.

If you don't run this step you'll need to provide the necessary toolchain.
The OpenTitan project distributes a [docker container](https://docs.opentitan.org/util/container/README/) with the toolchain preinstalled.

## Next steps

This was mostly meant as a proof-of-concept, and likely won't be taken much further.
Hopefully, this can help inform decisions regarding OpenTitan's [consideration to use Rust](https://github.com/lowRISC/opentitan/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+rust) for future software development.

That said, there are still some missing features. Primarily, this is supposed to implement a UART echo routine, but the UART appears misconfigured for reads.
So, a bit of debugging, and perhaps IRQ intergration. Aside from that, this code likely won't gain any additional features.

Outstanding questions:

 * `svd2rust` generates single-bit-writrs which don't require use of `unsafe`. However, multi-byte writers are marked `unsafe`.
    Is it possible to avoid this for registers to which any value can be written?
    (Compare [UART baud rate multiplier](https://nholstein.github.io/earlgrey_registers/uart/ctrl/struct.NCO_W.html#method.bits) to [UART date write](https://nholstein.github.io/earlgrey_registers/uart/wdata/index.html).)

 * ~~The OpenTitan `hello_world.elf` binary is 67K, compared to `earlgrey_uart.elf` weighing in at 1.1M, when `hello_world` has many more features.~~
   The small bits of disassembled Rust instructions I've reviewed seem space efficient and wouldn't explain the difference.
   Perhaps there's some necessary GC to perform during linking?

   The size discrepancy was caused by lack of stripping symbols; additionally the linker wasn't garbage collecting unused sections.

 * Meson vs. Make?

   I've spend some time playing around with trying to internalize as much of the build as possible into Cargo itself.
   This involved some experimentation into various features of Cargo which in some instances aren't clearly documented.
   I've written up [some notes from this](BUILD.md).

 * `svd2rust` doesn't propagate any comments/license through to the generated source. Is it worth adding this?
