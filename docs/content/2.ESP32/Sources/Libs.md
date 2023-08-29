# Libs


## Root Org

First we have the top level git organisation for esp32 / rust

  * https://github.com/esp-rs
    Root organisation for esp32 / rust code


## HAL Libs

Next we have HAL which can be used to access the gpio / lower level functions

  * https://github.com/esp-rs/esp-idf-hal
    Low level HAL Access such as gpio
    This one uses the idf so is a better choice generally

  * https://github.com/esp-rs/esp32-hal
    Low level HAL Access such as gpio - does not use the idf
    Can be used in situations where wifi etc isn't needed


## ESP IDF

For the ESP IDF we have

  * https://github.com/esp-rs/esp-idf-svc
    Bindings for esp-idf, these use esp-idf-sys


## Lower Level Depends

For Dependencies we have

  * https://github.com/esp-rs/esp32
    SVD files for the ESP32 - I think these are used by esp-idf-hal

  * https://crates.io/crates/esp-idf-sys
    Library that builds out the esp-idf
    I think this is used by esp-idf-svc

  * https://github.com/esp-rs/embedded-svc
    Base / common code for esp-idf-svc


## FreeRTOS

  * https://github.com/lobaro/FreeRTOS-rust
    FreeRTOS wrapper


## Older Libs

  * https://github.com/esp-rs/esp32-wifi
    I think this is now replaced by esp-idf-svc

