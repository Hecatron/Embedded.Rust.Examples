# Todo

## Remote Debuging

  * look at the runner under .cargo/config
  * does println work via jtag?

See if we can debug while setting ESP32_AFTERFLASH to no_reset
then running esptool.py 0p COM4 run to start it up
to capure / start the debug on the start of the code

## std

currently println doesn't work for the esp32
might be related to the different toolchain target xtensa-esp32-espidf
https://github.com/esp-rs/esp-idf-template/blob/master/cargo/src/main.rs


https://github.com/esp-rs/rust - fork i'm using

Need to study this next, it seems to be similar to the fork I'm using but uses
"xtensa-esp32-espidf" instead of "xtensa-esp32-none-elf" as the target

https://github.com/esp-rs/rust-build - binary build of the above, but also including idf?
https://github.com/ivmarkov/rust-esp32-std-demo

