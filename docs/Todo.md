# Todo

## ESP32

  * https://doc.rust-lang.org/beta/rustc/targets/custom.html
  * https://doc.rust-lang.org/stable/cargo/reference/overriding-dependencies.html#the-patch-section

see if cargo patch can be used to override compiler builtins

## Patch

This looks promising

https://github.com/mettke/cargo-patch

## cfg target

  * https://doc.rust-lang.org/reference/attributes.html
  * https://stackoverflow.com/questions/52794498/how-do-you-conditionally-compile-based-on-a-target-triple
  * https://stackoverflow.com/questions/41742046/is-there-a-list-of-all-cfg-features
  * https://stackoverflow.com/questions/48967583/how-to-get-executables-full-target-triple-as-a-compile-time-constant-without-us

note doesn't work with vscode as we don't have a custom rustc option yet

## Other Links

  * https://github.com/espressif/rust-esp32-example
  * https://github.com/orgs/esp-rs/repositories
  * https://github.com/esp-rs/esp-idf-svc
