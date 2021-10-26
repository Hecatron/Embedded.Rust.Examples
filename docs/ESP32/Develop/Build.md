# Building

## Build Systems

Currently there are 3 different build systems for rust

  * cargo - the original build system
  * xargo - a modified version of cargo for embedded builds (depreciated)
  * cargo xbuild - a variant based on xargo (depreciated)

xargo is now under maintenance mode so should be avoided for now.
cargo-xbuild replaced xargo and used to be the best option for the esp32

Native cargo now seems to work ok for the esp32 due to some underlying lib changed.
So it's best to just use this now.

## Switching toolchains

first we need to make sure we're on the esp toolchain
```
# To switch to the custom esp toolchain that is installed
rustup default esp
# to switch back to the stable x86 toolchain
rustup default stable
``


## Building

Building should be as simple as the following within the source directory
```
# Set Backtrace, this displays additional info on build / test failure
set export RUST_BACKTRACE=1

# To clean
cargo clean

# To Build - debug
cargo build
# To Build - release (smaller)
cargo xbuild --release
```

If we want to use some of the cutting edge features of cargo then we can use the following before hand
```
rustup default nightly
```


### Minimising build size

The first way to minimise the build size is by building in release instead of debug mode.

The next is the use of the panic_immediate_abort / panic_abort features for build-std
This seems to nock off a few Kb in terms of the final binary size
```
cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release
```

TODO I've not tried all of these just yet.

  * https://github.com/johnthagen/min-sized-rust
