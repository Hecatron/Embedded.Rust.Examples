# Readme

This rust library represents the bindings to the esp-idf
```
# install bindgen
cargo install bindgen

# generate bindongs
cargo make bindgen

# To do a build
cargo make build
```

## TODO

  * First we need the path to libclang.dll setting within LIBCLANG_PATH
    Currently I'm using the default mainline install of clang installed under
    "C:\Program Files\LLVM\bin"
    This needs to be added to the list of things to install
