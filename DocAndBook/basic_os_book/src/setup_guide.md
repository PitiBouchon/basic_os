# Setup

## Dependencies

In order to build and run the os you'll need :
- 🦀 [Rust](https://www.rust-lang.org/tools/install)

-  [cargo-make](https://github.com/sagiegurari/cargo-make) 
Install it via `cargo install --force cargo-make`

-  [QEMU](https://www.qemu.org/download/) [^note]

- Linux : if you want to use gdb on your machine you'll need the Newlib cross-compiler of [Riscv-GNU-Toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain)

[^note]: for RiscV

## Build & Run

Usual command `cargo build` and `cargo run` should work

Note that the `cargo run` command does `cargo make qemu` (see `.cargo/config.toml`) which is defined in `Makefile.toml`

## Debugging with gdb

Run in a terminal `cargo make qemu-gdb` \
In another terminal (in the directory containing the .gdbinit) run `riscv64-unknown-elf-gdb`
