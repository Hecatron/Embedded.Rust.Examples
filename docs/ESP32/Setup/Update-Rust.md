# Updating Rust


## Updating globally installed cargo utils / packages

We can globally update utils or cargo crates that have been installed by using cargo-update
```
# Make sure this is installed
cargo install cargo-update

# Update all global crates
cargo install-update -a
```


## Updating the toolchain

This isn't that relevant for the ESP32 since we are using a custom built version of the toolchain
But the following will update any default / native versions of the toolchain and rustup itself.
Also I think it will update cargo.
```
# Update the rust compiler
rustup update
```
