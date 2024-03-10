# Readme

This is just an updated fork of esp-idf-bindgen
Specifying updated depends for the creation of bindings against the ESP-IDF

  * https://github.com/reitermarkus/esp-idf-bindgen
  * http://reitermark.us/esp-idf-bindgen/esp_idf_bindgen/index

## Building

### Download IDF Sources

First we need the esp-idf sources
```
git clone https://github.com/espressif/esp-idf.git
cd esp-idf
git checkout v4.2
```

### Build Library

```
# Set the paths for rustc for the esp32
set XARGO_RUST_SRC=D:\SourceCode\External\rust-xtensa\library
set RUSTC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
set RUSTDOC=D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc

# Set the path for the downloaded idf
set export IDF_PATH=D:\SourceCode\External\esp-idf

# Build the library
cargo xbuild
```
