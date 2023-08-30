# Setup

## Installing Toolchain

First we need to install / setup the toolchain
```sh
# First install espup
# From https://github.com/esp-rs/espup
cargo install espup

# Next run update from espup
espup update
```

## Using the Toolchain

There are two ways to use the esp toolchain

The first method is to change the default toolchain in use
```sh
# To switch to the custom esp toolchain that is installed
rustup default esp
# to switch back to the stable x86 toolchain
rustup default stable
```

The second method is to create a file in the root of the project `rust-toolchain.toml`
With the following content in
```toml
[toolchain]
channel = "esp"
```

## Todo

  * https://github.com/esp-rs/esp-idf-template#prerequisites

```sh
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
```

```sh
cargo install cargo-generate
```
