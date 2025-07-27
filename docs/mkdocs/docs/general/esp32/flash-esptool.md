# Flashing with ESP Tool

  * https://github.com/ctron/rust-esp32-hono
    An example of some command line options here
  * https://github.com/esp-rs/esp32-hal/blob/master/flash
    An example of a wrapper script

  * https://github.com/sagiegurari/cargo-make
    TODO


## Flashing the Firmware

To flash with the esptool.py
First we need to convert the binary from an elf into a flashable binary using the elf2image subcommand
```
esptool.py --chip esp32 elf2image target/xtensa-esp32-none-elf/release/blinky
```

Next to flash the binary which should be the same as the built file but with the .bin exension
```
esptool.py --chip esp32 --baud 115200 --port COM4 --before default_reset --after hard_reset write_flash -z --flash_mode dio --flash_freq 40m --flash_size detect 0x1000 ../bootloader/esp32-bootloader.bin 0x8000 ../bootloader/partitions.bin 0x10000 target/xtensa-esp32-none-elf/release/blinky.bin
```

  * blinky.bin - This is the output from elf2bin to be flashed
  * esp32-bootloader.bin - This is the bootloader for the esp32 as an example
  * partitions.bin - This is a compiled version of the partition table to use

For the bootloader and the partition table these can be sourced from
https://github.com/esp-rs/espflash/tree/master/espflash/bootloader
Or the esp-idf repo, with esp-idf normally the partition table csv is compiled into a bin as part of that build process.


## Determine Onboard Flash size

One feature of esptool is to lookup the flash size / get the model id
We can also lookup the flashid within https://review.coreboot.org/cgit/flashrom.git/plain/flashchips.h
```
esptool.py -p COM2 flash_id
```


## Erasing the Flash

Another feature is the ability to erase the flash.
The advise is to hold down the boot / flash button then run the below, but I've found that usually isn't required
```
esptool.py --chip esp32 --port COM2 erase_flash
```