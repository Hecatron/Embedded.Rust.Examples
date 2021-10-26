# Flashing the Binary

Typically with the ESP32-WROVER-B Kit board there will be two COM Ports visible under windows, e.g. COM1, COM2
The first is used for Jtag, the second for uploading the flash

To determine the size of the flash try doing a release build then looking at the final binary
```
cargo build -- release
# e.g. see size of target/xtensa-esp32-none-elf/release/blinky
```


## Cargo ESPFlash

I've found the best way to flash a rust binary is to use the cargo espflash tool.
This takes care of handling the bootloader for you.
Typically this also builds the source at the same time
The tool option is to let it know which tool to use for building such as cargo, xbuild, xargo.

To upload a release version of the code just add --release to the command line options
The default is a debug build / upload which can be useful for jtag debugging.

I've also discovered that sometimes a reset of the board by trying to just access it via esptool is required.
I think this might be related to the set of the DTR pin I'm not sure.

```
# Sometimes this is needed to reset the board / put it into a state where cargo espflash will work
esptool.py -p COM2 flash_id
# Perform the flash
cargo espflash --chip esp32 --tool="xbuild" COM2
```


## esptool.py

### Determine Flash size

One feature of esptool is to lookup the flash size / get the model id
We can also lookup the flashid within https://review.coreboot.org/cgit/flashrom.git/plain/flashchips.h
```
esptool.py -p COM2 flash_id
```

### Erasing the Flash

Another feature is the ability to erase the flash.
The advise is to hold down the boot / flash button then run the below, but I've found that usually isn't required
```
esptool.py --chip esp32 --port COM2 erase_flash
```

### Flashing the firmware

Currently I would recommend using the cargo espflash tool to upload the main body of the firmware.
In order to use esptool.py for this I think would require manually downloading some bootloaders and putting them at the correct address
Also the creation of a partition table.

  * https://github.com/esp-rs/esp32-hal/blob/master/flash - an example of a wrapper script
  * https://github.com/ctron/rust-esp32-hono - an example of some command line options here

I think this needs a bootloader / partition table as well
stick with cargo espflash for now since it's easier
```
esptool.py --chip esp32 --port COM4 write_flash -z 0x1000 target\xtensa-esp32-none-elf\release\blinky
```
