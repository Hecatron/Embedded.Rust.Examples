# Todo

## Config

  * try different debuggers for launch.json - CodeLLDB / Native Debug
    So far Cortex Debug seems to work
  * look at the runner under .cargo/config
  * python wrapper to set the env variables for vscode
  * test the jlink ocd setup
  * does println work via jtag?

  * set env vars via cargo-make

## Switch from cargo xbuild to cargo build

Currently there's an issue with the mem functions being duplicated between esp32-hal / compiler-builtins
So to get around this we use cargo-xbuild instead of cargo

  * https://github.com/MabezDev/xtensa-rust-quickstart/issues/37
  * Once we're on cargo rename .config/config to .config/config.toml


## Size Difference

Once we switch to cargo build-std, see if there's still a 100Kb size difference in the final binary
Cargo includes the following additional libs or it might be the additional memory functions being linked in
```
   Compiling core v0.0.0 (D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\src\rust\library\core)
   Compiling rustc-std-workspace-core v1.99.0 (D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\src\rust\library\rustc-std-workspace-core)
```
Size increased from 473Kb to 573Kb for release


## Flashing tool broken

currently cargo espflash has issues under windows

  * https://lib.rs/crates/cargo-espflash

```
cargo espflash COM4
cargo espflash board-info COM4 
```


TODO this works
make a python wrapper script

https://github.com/ctron/rust-esp32-hono

esptool.py --chip esp32 elf2image target/xtensa-esp32-none-elf/release/blinky
esptool.py --chip esp32 --baud 115200 --port COM4 --before default_reset --after hard_reset write_flash -z --flash_mode dio --flash_freq 40m --flash_size detect 0x1000 ../bootloader/esp32-bootloader.bin 0x8000 ../bootloader/partitions.bin 0x10000 target/xtensa-esp32-none-elf/release/blinky.bin



# Ram Boot doesn't work?
esptool.py --chip esp32 --baud 115200 --port COM4 load_ram target/xtensa-esp32-none-elf/release/blinky.bin
xtensa-esp32-elf-objdump -h target\xtensa-esp32-none-elf\release\blinky
esptool.py --chip esp32 image_info target\xtensa-esp32-none-elf\release\blinky.bin


flash entry is 4007c6a0
ram entry is   3f400020



CARGO_MAKE_CRATE_TARGET_DIRECTORY
CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY
CARGO_MAKE_CRATE_TARGET_TRIPLE


https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17



  * format has issues with esp when run from cargo make
  * build calls - "cargo" "build" "--all-features", can we prevent this?

