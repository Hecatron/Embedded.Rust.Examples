# Building

## Build Systems

Currently there are 3 different build systems for rust

  * cargo - the original build system (depreciated)
  * xargo - a modified version of cargo for embedded builds (depreciated)
  * cargo xbuild - a variant based on xargo

xargo is now under maintenance mode so should be avoided for now.
cargo-xbuild has replaced xargo and is what I'm using at the moment for the esp32

However cargo-xbuild recommends using the original cargo in build-std mode
Unfortunatly I've not quite managed to get build-std mode working yet for the esp32
So cargo xbuild is used here untill the bugs are sorted out

In order to build the source from the command line

## Setup

First the setup
```
# Note if we decide to switch to cargo build instead of cargo.xbuild later on
# Then the nightly version of cargo is needed for custom rustc to be used via .config/config.toml
rustup default nightly

# For now use cargo-xbuild until the mem issue with esp32-hal / compiler-builtins is fixed
cargo install cargo-xbuild

# Make sure this is installed on the system
rustup component add rust-src
```


## Building

To build
```
# Set Backtrace
set export RUST_BACKTRACE=1

# To clean
cargo clean

# We should be able to get rid of these once these can be placed into .cargo/cargo.toml
# when using cargo build instead of cargo xbuild
set XARGO_RUST_SRC=D:\SourceCode\External\rust-xtensa\library
set RUSTC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
set RUSTDOC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc

# To build debug
cargo xbuild
# To buld release
cargo xbuild --release
```

### Minimising build size

So far I've not managed to get all of these to work for the ESP32
Probably due to the use of xbuild and not build-std

  * https://github.com/johnthagen/min-sized-rust
