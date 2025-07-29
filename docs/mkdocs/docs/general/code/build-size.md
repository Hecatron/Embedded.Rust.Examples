# Build Size

Making a build size as small as possible can be important with an embedded platform.  
The first way to minimise the build size is by building in release instead of debug mode.

The next is the use of the panic_immediate_abort / panic_abort features for build-std
This seems to nock off a few Kb in terms of the final binary size
```bash
cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release
```

TODO I've not tried all of these just yet.

  * <https://github.com/johnthagen/min-sized-rust>
