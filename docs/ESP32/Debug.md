# Debugging

## Flashing

To upload to the board (COM port should be the second one out of the two)
```
cargo espflash --chip esp32 --tool="xbuild" COM4
```

Sometimes before connecting via the cargo espflash module I find things only work if I try running the esptool.py first
```
esptool.py -p COM4 flash_id
```


TODO

  * https://demo-dijiudu.readthedocs.io/en/latest/api-guides/jtag-debugging/tips-and-quirks.html


## JTag (dev board)

### WinUSB Driver

Typically there are 2 serial ports on the ESP32 dev board
The first is for Jtag, the second is for flashing the device
To use Jtag the first needs to have it's driver swapped with WinUSB

  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/jtag-debugging/configure-ft2232h-jtag.html

The Winusb driver is HID not serial
so once replaced the device will show up as a HID instead of a serial port

A reboot may be needed afterwards in order for the serial port for flashing to continue working

### JTag Jumpers

  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-wrover-kit.html
