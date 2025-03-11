# Probe-rs Notes

Currently probe-rs isn't stable enough to work with the esp devices yet

## Flashing with probe-rs

In order to flash with probe-rs you need to switch the serial port driver with WinUsb using zadig
To flash
```
cargo embed
```

## Boards

### ESP-WROVER-KIT

For the ESP-WROVER-KIT there were two serial ports, for me trying this with the first port - interface0 seems to work
listed as FTDIBUS (v2.12.28.0) by default

### ESP-WROOM-32

This is one of the cheap china boards, probe-rs didn't seem to recognise this as acceptable

  * CP2102 USB to UART Bridge Controller
  * USB\VID_10C4&PID_EA60\0001

## ESP TTGo Board

This is another china esp32 board
also not recognised

  * CP2104 USB to UART Bridge Controller
  * USB\VID_10C4&PID_EA60\018A8812

### Restoring driver from zadig

To restore the original driver:

  * https://github.com/pbatard/libwdi/wiki/FAQ#Help_Zadig_replaced_the_driver_for_the_wrong_device_How_do_I_restore_it

## TODO

From the looks of things with probe-rs

  * Flashing sort of works, but the probe-rs halts / pauses after the flash as does the esp32
    requiring a unplug / plugin of the device to reset
  * For debugging attaching works but not launching I think due to the above
  * single stepping doesn't work but breakpoints does
  * RTT logging doesn't work
  * For launch.json The svd file?
