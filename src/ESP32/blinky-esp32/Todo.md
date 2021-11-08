# Todo

Make sure this link is in the docs
https://github.com/ctron/rust-esp32-hono
https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17
https://github.com/lexxvir/esp32-hello
https://github.com/MabezDev/xtensa-rust-quickstart


## Remote Debuging

  * look at the runner under .cargo/config
  * does println work via jtag?
  * set env vars via cargo-make

See if we can debug while setting ESP32_AFTERFLASH to no_reset
then running esptool.py 0p COM4 run to start it up
to capure / start the debug on the start of the code
