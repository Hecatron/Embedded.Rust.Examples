# Libs

## Frameworks

Embassy seems more fully fleshed out and has more support for different platforms.
But apprarantly it's possible to mix and match between the two

Embassy

  * https://embassy.dev/dev/index.html
    https://dev.to/apollolabsbin/embedded-rust-embassy-uart-serial-communication-4fd3
    https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown
  * https://www.youtube.com/watch?v=vwFWBP0IuRU
  * https://nereux.blog/posts/esp32-ws2812-dino-light-2/

Rtic

  * https://github.com/rtic-rs/rtic
    Mostly Cortex-M only

TockOS

  * https://tockos.org/

## Other Libs

  * Tonic - grpc
    https://github.com/hyperium/tonic/tree/master

## Serial Port Libs

  * Cross platform serial port lib
    https://lib.rs/crates/serialport
  * Serial port over a network connection
    https://github.com/sjm42/net-serial-console-rs

Note we can't use tokio serial for async as it doesn't support nostd platforms
https://github.com/tokio-rs/tokio/discussions/4617
