# Build System

  * https://rust-lang.github.io/rustup/examples.html
  * https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html

## Toolchains

Toolchains are variants of the build system.
To install a toolchain other than stable
```
rustup toolchain install stable
rustup toolchain install beta
rustup toolchain install nightly
```

To set the toolchain
```
rustup default stable-msvc
rustup default nightly
```


## Components

Components represent individual parts of the toolchain for targets that can be installed
```
rustup component list
```
