# Readme

Example accessing sdmmc from ESP32 using Fat32

## Libs

  * https://github.com/rafalh/rust-fatfs

## TODO

### SD Card

Access SD card from esp32

  * ESP-WROVER-KIT is wired for Slot 1 for the external SD Card
  * 4 Line is available
  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/peripherals/sdmmc_host.html
  * https://dl.espressif.com/dl/schematics/ESP-WROVER-KIT_V4_1.pdf
  * https://github.com/micropython/micropython/issues/4722

### Wifi

  * https://github.com/jeikabu/esp_idf/ - small wrapper for idf
  * https://github.com/esp-rs/esp32-wifi - rust based
  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html

### ESP IDF Bindings

  * https://github.com/reitermarkus/esp-idf-bindgen - idf binding generator
  * https://github.com/rust-lang/rust-bindgen - general binding generator


## SPI Ports

There are currently 4 SPI Ports

  * https://docs.espressif.com/projects/esp-idf/en/release-v3.0/api-reference/peripherals/spi_master.html
  * https://randomnerdtutorials.com/esp32-pinout-reference-gpios/
  * https://electronics.stackexchange.com/questions/451707/how-to-define-the-spi-pins-in-an-esp32-module
  * https://www.neonious.com/Blog/index.php/2019/12/05/esp32-wrover-with-8-mb-64-mbit-psram-better-in-unexpected-badly-documented-way-hint-spi/

  * SPI0 - Used for reading Flash - gpio 6,7,8,9,10,11 - also used by psram / gpio16
  * SPI1 - Used for writing flash / getting the flash id
  * HSPI - Used by JTag / SDMMC - gpio 12,13,14,15
  * VSPI - gpio 5(CS), 18(CLK), 19(MISO), 23(MOSI)

VSPI seems to be the only spare one if using an SD card and Jtag at the same time
I think this is shared with the LCD but has a seperate chip select


TODO

Setup a sdmmc project that can ether access an SD card via VSPI for when Jtag is running
or SDMMC when jtag is not running for faster SD card speed


### SPI Speed

I think the default speed is 40Mhz / 40Mbps
But can be upped to 80Mhz if using the hardware SPI interfaces

  * https://blog.lvgl.io/2019-01-31/esp32

Also we need to use DMA with SPI so that the CPU isn't kept busy
https://github.com/espressif/esp-idf/issues/2128


## JTag

Currently there is a conflict while trying to use SDMMC (4wire) and JTag at the same time
Typically the ESP32 dev board needs a jumper to be set to enable the onboard Jtag so that it doesn't conflict with the onboard SD card slot

https://www.esp32.com/viewtopic.php?t=7141
https://www.reddit.com/r/esp32/comments/d71es9/a_breakdown_of_my_experience_trying_to_talk_to_an/


For jtag swd is not supported
pins are 12,13,14,15
https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/jtag-debugging/configure-other-jtag.html
