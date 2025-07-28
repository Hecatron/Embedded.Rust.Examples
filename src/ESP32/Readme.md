# Readme

For the ESP32 chipsset it's possible to use std or nostd

  * The nostd examples use esp-hal which is rust only
  * The std examples esp-idf-hal / esp-idf which uses the esp libraries which are written in C
  * https://github.com/esp-rs/esp-hal/discussions/1288

Generally the std / esp-idf-hal is easier to get up and running
But esp-hal is pure rust
