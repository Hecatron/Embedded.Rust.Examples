# Todo

## Remote Debuging

  * look at the runner under .cargo/config
  * does println work via jtag?

See if we can debug while setting ESP32_AFTERFLASH to no_reset
then running esptool.py 0p COM4 run to start it up
to capure / start the debug on the start of the code

## No std

To avoid usind std for a smaller file

Add the following to the top of your main rust code
```
#![no_std]
```

For cargo.toml use something like
```
[unstable]
build-std = ["core", "alloc"]
```

For Cargo.toml we also need panic-halt as a depend
```
panic-halt = "0.2.0"
```
