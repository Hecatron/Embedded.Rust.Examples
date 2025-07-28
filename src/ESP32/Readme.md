# Readme

For the ESP32 chipsset it's possible to use std or nostd

## NoStd

The nostd examples use esp-hal which is rust only
This is based on `esp-generate`

## Std

The std examples use esp_idf_svc, this builds upon esp-idf-hal / esp-idf
esp-idf-hal uses the uses the native esp libraries which are written in C

  * https://github.com/esp-rs/esp-hal/discussions/1288

Generally the std / esp-idf-svc is easier to get up and running
But esp-hal is pure rust

esp-idf-svc is used for higher level applications such as wifi / bluetooth etc
esp-idf-hal is used for lower level applications such as setting an LED / SPI

  * https://github.com/esp-rs/esp-idf-hal/tree/master/examples
  * https://github.com/esp-rs/esp-idf-svc/tree/master/examples

This is based on
```sh
cargo generate esp-rs/esp-idf-template cargo
```
