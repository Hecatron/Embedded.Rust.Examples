---
title: "Rpi Pico Degug Probe"
---

  * https://www.raspberrypi.com/products/debug-probe/
  * https://datasheets.raspberrypi.com/debug/debug-connector-specification.pdf
  * https://github.com/raspberrypi/debugprobe
  * https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html

The Rpi Debug probe low cost AWD Debug probe that uses the CMSIS-DAP standard.
It's powered by a RP2040 and can provide SWD Debugging for ARM based micro's similar to the Segger.
It also has a separate UART Port for serial communication.

## Ports

There are two 3 pin ports

### UART Port

This is labelled as `U` - UART Serial Port

  * Red/Orange - TX
  * Black - Ground
  * Yellow - RX

### Debug Port

This is labelled as `D` - Debug SWD Port

  * Red/Orange - SC / SWD Clock
  * Black - Ground
  * Yellow - SD / SWD Data Line
