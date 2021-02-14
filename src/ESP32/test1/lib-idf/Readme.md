# Readme

Test build for the idf
There are 3 steps to this build currently

```
# Sets up the python virtual environment
cargo make pyenv
# Triggers a cmake setup under the build directory
cargo make cmake
# Runs ninja to do the build
cargo make build
```

The idf can also be configured via
```
cargo make menuconfig
```


## Install idf

```
git clone https://github.com/espressif/esp-idf.git
cd esp-idf
git checkout v4.2
git submodule update --init --recursive
```

## Dependencies

  * CMake
  * Ninja build system
    https://github.com/ninja-build/ninja/tags
  * python


## TODO

Need to look at https://github.com/lexxvir/esp32-hello
Is there a way to just build lib-idf as a library we can link against
and exclude the partition table / bootloader?
