# Todo

Make sure this link is in the docs
https://github.com/ctron/rust-esp32-hono
https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17
https://github.com/lexxvir/esp32-hello
https://github.com/MabezDev/xtensa-rust-quickstart


## Remote Debuging

  * try different debuggers for launch.json - CodeLLDB / Native Debug
    So far Cortex Debug seems to work
  * look at the runner under .cargo/config
  * python wrapper to set the env variables for vscode
  * test the jlink ocd setup
  * does println work via jtag?
  * set env vars via cargo-make

See if we can debug while setting ESP32_AFTERFLASH to no_reset
then running esptool.py 0p COM4 run to start it up
to capure / start the debug on the start of the code

Current cortex-debug seems to work, may need a bit of tweaking


## Ram Boot

Ram booting doesnt work?

esptool.py --chip esp32 --baud 115200 --port COM4 load_ram target/xtensa-esp32-none-elf/release/blinky.bin
xtensa-esp32-elf-objdump -h target\xtensa-esp32-none-elf\release\blinky
esptool.py --chip esp32 image_info target\xtensa-esp32-none-elf\release\blinky.bin

flash entry is 4007c6a0
ram entry is   3f400020
