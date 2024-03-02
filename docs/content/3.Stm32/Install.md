# Install

## ARM Target

For the dependencies first we need the arm target for the stm32
```sh
rustup target add thumbv7em-none-eabihf
```

## Flash Tools

Next we need to install binstall for using cargo to do binary installs
```sh
cargo install cargo-binstall
```

Finally we need probe-rs, this includes flashing and debugging utils
```sh
cargo binstall probe-rs
```

## VSCode debugger

For VSCode install the extension "Debugger for probe-rs"

  * https://probe.rs/docs/tools/debugger/#using-the-launch-request-type
