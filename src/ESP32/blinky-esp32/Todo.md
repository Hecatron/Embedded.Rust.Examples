# Todo

Make sure this link is in the docs
https://github.com/ctron/rust-esp32-hono
https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17
https://github.com/lexxvir/esp32-hello
https://github.com/MabezDev/xtensa-rust-quickstart


## VSCode

  * Issues with vscode rust-analyser

## Building

  * build calls - "cargo" "build" "--all-features", can we prevent this?
    Or set it to not do anything?

## Code Formatting

  * format has issues with esp when run from cargo make

## Remote Debuging

  * try different debuggers for launch.json - CodeLLDB / Native Debug
    So far Cortex Debug seems to work
  * look at the runner under .cargo/config
  * python wrapper to set the env variables for vscode
  * test the jlink ocd setup
  * does println work via jtag?
  * set env vars via cargo-make


## Ram Boot

Ram booting doesnt work?

esptool.py --chip esp32 --baud 115200 --port COM4 load_ram target/xtensa-esp32-none-elf/release/blinky.bin
xtensa-esp32-elf-objdump -h target\xtensa-esp32-none-elf\release\blinky
esptool.py --chip esp32 image_info target\xtensa-esp32-none-elf\release\blinky.bin

flash entry is 4007c6a0
ram entry is   3f400020
