# Readme

For the ESP32 chipsset it's possible to use std or nostd

## NoStd

The nostd examples use esp-hal which is rust only
This is based on `esp-generate`

## Std

The std examples use esp_idf_svc, this builds upon esp-idf-hal / esp-idf
which uses the esp libraries which are written in C

  * https://github.com/esp-rs/esp-hal/discussions/1288

Generally the std / esp-idf-svc is easier to get up and running
But esp-hal is pure rust

This is based on
```sh
cargo generate esp-rs/esp-idf-template cargo
```
