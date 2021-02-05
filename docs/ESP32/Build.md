# Building


## Setup

First the setup
```
# Note the nightly version of cargo is needed for custom rustc to be used via .config/config.toml
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

# We should be able to get rid of these once cargo is working
set export RUST_BACKTRACE=1 
set XARGO_RUST_SRC=D:\SourceCode\External\rust-xtensa\library
set RUSTC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
set RUSTDOC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc

# To build debug
cargo xbuild
# To buld release
cargo xbuild --release
```
