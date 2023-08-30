# Targets

Each target is a different type of CPU / chip arrangement
By default x86_64-pc-windows-msvc will be the one installed under windows

But rust is a cross compiler by default.
So can build for any target installed


## List Targets

To list all available targets
```
rustup target list
```

## Install new target

To install a new target
This installs the rust-std component for the thumbv7em-none-eabihf target
```
rustup target add thumbv7em-none-eabihf
```

## Build for Target

To build a project using the target
```
cargo build --target=thumbv7em-none-eabihf
```

## Custom Targets

  * https://doc.rust-lang.org/rustc/targets/custom.html
  * https://book.avr-rust.com/005.1-the-target-specification-json-file.html

Custom targets are defined by a json file.

To view the json of a esp32 target
```
# -Z unstable-options is needed to print the target json
# The version of rustc is from the fork built for the esp32
rustc -Z unstable-options --target xtensa-esp32-none-elf --print target-spec-json

# Output
{
  "arch": "xtensa",
  "cpu": "esp32",
  "data-layout": "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32",
  "emit-debug-gdb-scripts": false,
  "env": "",
  "executables": true,
  "is-builtin": true,
  "linker": "xtensa-esp32-elf-gcc",
  "linker-flavor": "gcc",
  "llvm-target": "xtensa-none-elf",
  "max-atomic-width": 32,
  "os": "none",
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-pointer-width": "32",
  "unsupported-abis": [
    "stdcall",
    "fastcall",
    "vectorcall",
    "thiscall",
    "win64",
    "sysv64"
  ],
  "vendor": ""
}
```

Typically it defines the location of the linker and compiler options for llvm
It doesn't specify the rustc compiler