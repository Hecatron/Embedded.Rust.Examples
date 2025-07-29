# Install of Tools

This assumes you already have `rustup` already installed on your windows or linux operating system.

## Toolchain Setup

Note it's probably best to stick with the stable toolchain
unless there's some nightly feature you want to actually use

To install stable / nightly tobasholchains
```bash
rustup toolchain install stable
rustup toolchain install nightly
```

To switch the default toolchain (globally)
```bash
rustup default nightly
rustup default stable
```

To switch between toolchains for a directory / project basis
```bash
rustup override set nightly
rustup override set stable
```

## Useful Tools

Install some useful tools
```bash
cargo install cargo-generate
cargo install cargo-make
cargo install cargo-binstall
```

For web assembly
```bash
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
```

This is required by rust analyser I think
```bash
rustup component add rust-src
```

## ESP32 Development

  * <https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html>

```bash
cargo install espup
espup install
cargo install ldproxy

# Add the following line to .bashrc
. $HOME/export-esp.sh
```

## STM32 Development

```bash
cargo install cargo-binutils
rustup component add llvm-tools
rustup target add thumbv6m-none-eabi
rustup target add thumbv7m-none-eabi
rustup target add thumbv7em-none-eabi
rustup target add thumbv7em-none-eabihf
rustup target add thumbv8m.base-none-eabi
rustup target add thumbv8m.main-none-eabi
rustup target add thumbv8m.main-none-eabihf
```

## Rpi Pico

  * <https://www.alexdwilson.dev/how-to-program-raspberry-pi-pico-with-rust>

```bash
rustup target add thumbv6m-none-eabi
rustup target add riscv32imac-unknown-none-elf
cargo install elf2uf2-rs
```
