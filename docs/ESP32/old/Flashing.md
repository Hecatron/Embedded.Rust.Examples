# Flashing

Typically with the ESP32-WROVER-B board there will be two COM Ports visible under windows
e.g. COM3, COM4
the second one is used for flashing - COM4

I think the default built binary is target\xtensa-esp32-none-elf\debug\xtensa-quickstart
to get an idea of how big it actually is build in --release mode



## Cargo ESPFlash

To flash using cargo espflash
the tool option is to let it know which tool to use for building if building is required
```
cargo espflash --chip esp32 --example esp32 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" --tool="xargo" COM4
```


## Determine Flash size

To check the flash size, get the model id from
```
esptool.py -p COM4 flash_id
```

Then lookup under

  * https://review.coreboot.org/cgit/flashrom.git/plain/flashchips.h


## ESP Tool

TODO Currently this requires a seperate partition and bootloader file to work

To erase the flash
```
# Hold down the boot / flash button
# Then run the below to erase
esptool.py --chip esp32 --port COM4 erase_flash
```

The below doesn't currently work
```
esptool.py --chip esp32 --port COM4 write_flash -z 0x1000 target\xtensa-esp32-none-elf\debug\examples\esp32
```

See 

  * https://github.com/esp-rs/esp32-hal/blob/master/flash
    seems to be the best one, although sticking with cargo espflash for now
  * https://github.com/ctron/rust-esp32-hono
