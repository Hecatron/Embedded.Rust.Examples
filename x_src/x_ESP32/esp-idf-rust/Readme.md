# Readme

This is just an updated fork of esp-idf-bindgen

  * https://github.com/reitermarkus/esp-idf-bindgen
  * http://reitermark.us/esp-idf-bindgen/esp_idf_bindgen/index

There are two subprojects here

  * bindings-generator
    This looks at the esp-idf then generates the needed rust bindings file

  * esp-idf-rust
    Wrapper library that uses the generated bindings


## Setup

### Download IDF Sources

First we need the esp-idf sources
```
git clone https://github.com/espressif/esp-idf.git
cd esp-idf
# Specify which version we want
git checkout v4.2
```

### Download a esp32 toolchain

Download and extract a esp32 toolchain for windows

  * https://github.com/espressif/crosstool-NG/releases


## Running the bindings generator

This is within the bindings-generator directory

First we need to setup some enviromental variables
The easiest way to do this is to create a .cargo/env-settings.toml file which will be picked up by cargo make
Change the paths to match your own

.cargo/env-settings.toml
```
[env]
# Show debugging info on error of build
RUST_BACKTRACE = 1

# Set which target to generate the bindings for
TARGET = xtensa-esp32-none-elf

# Directory of downloaded esp-idf
IDF_PATH = D:\\SourceCode\\External\\esp-idf

# Sysroot directory of xtensa-esp32-elf tools
SYS_ROOT = C:\\Apps\\xtensa-esp32-elf\\xtensa-esp32-elf
```

Next build the bindings generator
```
cd bindings-generator
caro make
```

TODO run







## TODO
