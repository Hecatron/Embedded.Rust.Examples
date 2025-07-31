# Todo

under linux

## riscv Nano blinky

copy esp32-nostd\examples\wroom-kit-blinky
across to esp32-c6-nostd\examples\nano-c6-blinky
then setup for smart leds

## esp-idf-svc

Currently there's an issue with flashing the svc based images
seems to cause a reboot loop of the board

I think with one of the changes I've made it nearly works

Things to check
1. is the partition table needed?
2. does the version of idf need to be 5.3.2
3. does probe-rs work (I think it doesnt reset the board)
4. does rtt work? (need probe-rs to work first)

# Flashing with espflash causes the program to run
cargo espflash flash --example nano-c6-blinky --monitor
# using probe-rs it will flash but not run until the board is manually reset
cargo run --example nano-c6-blinky

If rtt output isn't showing after the above when flashing a nostd image
try unplugging / plugging the device back in

## std example

https://github.com/sjm42/esp32temp/tree/master
https://github.com/esp-rs/esp-hal/discussions/2947
