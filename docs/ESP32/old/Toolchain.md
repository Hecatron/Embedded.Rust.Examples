# Toolchain

In order to use rust with the ESP32 we need to setup a custom toolchain

  * https://github.com/MabezDev/xtensa-rust-quickstart

## Build toolchain

From a regular command line

```
cd D:\SourceCode\External
git clone https://github.com/MabezDev/rust-xtensa
cd rust-xtensa
CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
C:\Python38\python.exe src/bootstrap/configure.py --experimental-targets=Xtensa
C:\Python38\python.exe x.py build --stage 2
```

Currently using the latest e45940c1d4b03eb3e0761e2304c5238afd591d9f
which seems to build ok


## Use toolchain

TODO 

XARGO_RUST_SRC=\path\to\rust-xtensa\library
RUSTC=\path\to\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
