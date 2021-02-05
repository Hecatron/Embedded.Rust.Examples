# Build

I think its currently set for "cargo xbuild"
but I think this has been depreciated in favour of "cargo build-std"

## Links

  * https://github.com/japaric/xargo
  * https://github.com/rust-osdev/cargo-xbuild
  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/MabezDev/rust-xtensa
  * https://github.com/espressif/llvm-project
  * https://dentrassi.de/2019/06/16/rust-on-the-esp-and-how-to-get-started/

## Install Depends

### Xargo

First we need xargo
```
cargo installl xargo
```

### Esptool

Under python install the esptool for flashing
and the cargo esp flash tool
```
pip install esptool
cargo install cargo-espflash
```

### ESP32 Gcc

Download and extract the following, to somewhere like C:\Apps

  * https://github.com/espressif/crosstool-NG/releases
  * xtensa-esp32-elf-gcc8_4_0-esp-2020r3-win64.zip

Add the following to the path

  * C:\Apps\xtensa-esp32-elf\bin

## Blinky App

Next to build the app
```
git clone https://github.com/MabezDev/xtensa-rust-quickstart.git
cd xtensa-rust-quickstart

set export RUST_BACKTRACE=1 
set XARGO_RUST_SRC=D:\SourceCode\External\rust-xtensa\library
set RUSTC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
set RUSTDOC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc

# To build the code from src
xargo build --features="xtensa-lx-rt/lx6 xtensa-lx/lx6 esp32-hal"
xargo build --features="xtensa-lx-rt/lx6 xtensa-lx/lx6 esp32-hal" --release
# to build the esp32 example code from examples
xargo build --features="xtensa-lx-rt/lx6 xtensa-lx/lx6 esp32-hal" --example esp32 --release
```
