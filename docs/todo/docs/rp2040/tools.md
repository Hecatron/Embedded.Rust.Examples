---
title: RP2040 Tools
description: An overview of STM32 Development
---

Install the following
```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# This is our suggested default 'runner'
cargo install probe-run --locked
# If you want to use elf2uf2-rs instead of probe-run, instead do...
cargo install elf2uf2-rs --locked
# If you want to use any of the probe-rs tools (probe-rs run, cargo-embed, probe-rs-debugger)
cargo install probe-rs --features=cli --locked
```
