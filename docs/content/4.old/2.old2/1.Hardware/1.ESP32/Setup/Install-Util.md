# Install General Utils

## Debugging Tools

### Openocd - ESP32

For JTag debuggging one tool we need is the version of openocd built for the esp32
This can be used with the ESP32 Wrover Kit board

  * https://github.com/espressif/openocd-esp32/releases

For Windows for example extract it to some place such as
```
C:\Apps\dev-tools\openocd-esp32
```


## Util

### Cargo-Make

One very useful tool for rust is cargo-make
It can be used to add a wrapper around the cargo build process.

  * https://sagiegurari.github.io/cargo-make/

To install
```
cargo install cargo-make
```


## Obselete

  * cargo-xbuild - https://github.com/rust-osdev/cargo-xbuild
    Originally this was required for building rust for use with the ESP32
    However it's no longer required as native cargo support works now

  * rust-src
    This is useful for when you need to use cargo in non-std mode
    However we no longer need this anymore with the most recent tools
    rustup component add rust-src
