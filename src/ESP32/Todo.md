# Todo

under linux

## riscv Nano blinky

copy esp32-nostd\examples\wroom-kit-blinky
across to esp32-c6-nostd\examples\nano-c6-blinky
then setup for smart leds

## esp-idf-svc

Currently there's an issue with flashing the svc based images
seems to cause a reboot loop of the board
Dedbugging esp32-std / esp32-c6-std doesn't work, need to fix the above first

If rtt output isn't showing after the above when flashing a nostd image
try unplugging / plugging the device back in

## std example

https://github.com/sjm42/esp32temp/tree/master
https://github.com/esp-rs/esp-hal/discussions/2947
