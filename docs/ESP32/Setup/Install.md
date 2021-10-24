# Install

## Build Tools

### Cargo xbuild

  * https://github.com/rust-osdev/cargo-xbuild

Currently we need to use cargo xbuild instead of build with no-std due to a memory function conflict
between esp32hal and compiler builtins
```
cargo install cargo-xbuild
```

### rust-src

This can be needed if trying to use cargo in non-std mode
```
rustup component add rust-src
```

### ESP32 GCC

One of the tools we'll need is the version of gcc for the esp32
Download and extract the following then add to you're path

  * https://github.com/espressif/crosstool-NG/releases
  * For Windows: xtensa-esp32-elf-gcc8_4_0-esp-2021r2-win64.zip

For example if you extracted it to C:Apps\ then you'd need to add the following to your path under windows
C:\Apps\xtensa-esp32-elf\bin


## Flash Tools

### esptool.py

One of the tool we need is esptool.py
This is a python based tool so you'll need python installed to install and use it.
But it allows for querying / writing / clearing the flash.
```
pip install esptool
```

### Cargo espflash

The cargo espflash tool makes uploading flash files easier
since it doesn't require manually downloading bootloaders.
```
cargo install -f cargo-espflash
```


## Debugging

### Openocd - ESP32

For JTag debuggging one tool we need is the version of openocd built for the esp32
This can be used with the ESP32 Wrover Kit board

  * https://github.com/espressif/openocd-esp32/releases/tag/v0.10.0-esp32-20201202


## Util

This is a useful wrapper tool for cargo
```
cargo install cargo-make
```

  * https://sagiegurari.github.io/cargo-make/