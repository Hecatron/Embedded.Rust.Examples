# Debugging

For hardware gdb debugging we can ether use the Segger or for the ESP-WROVER-KIT board the inbuilt Jtag interface

  * https://demo-dijiudu.readthedocs.io/en/latest/api-guides/jtag-debugging/tips-and-quirks.html
  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/jtag-debugging/index.html
  * https://dzone.com/articles/eclipse-jtag-debugging-the-esp32-with-a-segger-j-l


## JTag (esp32 dev board)

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


## Jtag Segger

TODO
