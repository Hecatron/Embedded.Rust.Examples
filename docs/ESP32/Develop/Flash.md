# Flashing - General

Typically with the ESP32-WROVER-B Kit board there will be two COM Ports visible under windows, e.g. COM1, COM2
The first is used for Jtag, the second for uploading the flash

If the Jtag port has been switched with the HID driver for use with OpenOcd
then only one COM port will be visible

To determine the size of the flash try doing a release build then looking at the final binary
```
cargo build --release
esptool.py --chip esp32 elf2image target/xtensa-esp32-none-elf/release/blinky
# e.g. see size of target/xtensa-esp32-none-elf/release/blinky.bin
```

## Memory Map ESP32

  * https://www.lucadentella.it/en/2017/09/30/esp32-22-spiffs/
  * https://github.com/espressif/esptool
  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/bootloader.html
  * https://blog.espressif.com/esp32-programmers-memory-model-259444d89387



TODO
