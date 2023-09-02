# Setup

## Updating Rustup

First to update rustup and rust to the latest versions
```sh
rustup update
```

## Installing ESP32 Toolchain

For the esp32 this currently uses a separate toolchain.
To get the needed bits installed we can use `espup`
```sh
# First install espup
# From https://github.com/esp-rs/espup
cargo install espup
# Next run update from espup
espup update
```

Note if using windows terminal, make sure to restart it completely after running `espup update`

## Additional ESP32 related Tools

Looking at the current ESP32 template - https://github.com/esp-rs/esp-idf-template#prerequisites
It also mentions to install the following

```sh
cargo install cargo-generate
cargo install ldproxy
cargo install espflash
cargo install cargo-espflash # Optional
```

## Linux Notes

For linux first we need to make sure some debs are installed
```sh
sudo apt-get install -y gcc build-essential curl pkg-config
sudo apt-get install libudev-dev
```

Whenever running rust we need to make sure certain env variables are set
```sh
. /home/dietpi/export-esp.sh
```
