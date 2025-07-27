# Firmware Updates

In order to use probe-rs with the microbit I've found it's best to update the firmware
of the secondary processor (the `MKL26Z128VFM4`), the one that handles USB and Jtag to the latest version possible.

  * https://microbit.org/get-started/user-guide/firmware/

In order to update it

  * Hold the Reset button down while plugging in the device via USB.
  * It will show up as in `Maintenance` mode.
  * Copy across the Hex file in the above link to the mounted drive.
