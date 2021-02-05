# Scripts

From the looks of things it is possible to run rust as a script with a .rs file

  * https://rust-script.org/
  * https://neosmart.net/blog/2020/self-compiling-rust-code/

From the looks of things cargo-script was forked into rust-script since cargo-script was abandoned
It allows .rs files to be run without the need of Cargo.toml or src directories

  * https://www.reddit.com/r/rust/comments/jjnyv1/rustscript_run_rust_files_and_expressions_as/

## Install

To install
```
cargo install rust-script
```

## Run

To run
```
rust-script test-script.rs
```
