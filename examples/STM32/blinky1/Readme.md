# Readme

This basic example is targeted at the STM32F767ZI
for the NUCLEO-F767ZI board

This uses the below Hal library and uses this as an example
To build the below library outside of a dependency `cargo build --features=stm32f767,rt`

  * https://github.com/stm32-rs/stm32f7xx-hal

## Depends

The following target needs to be installed
```
rustup target add thumbv7em-none-eabihf
```

## Building

```
cargo build
```

## Flashing

Todo flashing / debugging

```
cargo install cargo-flash
```

