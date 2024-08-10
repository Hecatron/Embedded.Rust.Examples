# Cargo Commands

Note typically cargo deals with cargo libraries
rustup deals with the toolchains

## Init Project

Cargo is typically used to manage dependencies and can be used to call rustc to do the compilation
```
# Do an init in the current directory
cargo init
# Create a new project in a subdir
cargo new subdir
```

This will result in a cargo.toml file
Typically built files end up in target


## Run project

For building / running
Typically the built file ends up as 

  * target/debug/rust_sandbox
  * target/release/rust_sandbox

```
# To just build the project
cargo build
# to build then run the project
cargo run
# Build for production
cargo build --release
```

## Custom Registries / Packages

  * https://doc.rust-lang.org/cargo/reference/registries.html
  * https://rust-cli.github.io/book/tutorial/packaging.html
