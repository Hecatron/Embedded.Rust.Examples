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

It looks like the idf build system generates a final binary that includes a bootloader and partition table
See if we can exclude that from the build
look at the ninja build file for the targets in there

object files can be linked into a static library
but you cant really link static libs to one another
but you can split the obj files from a binary with ar
https://stackoverflow.com/questions/2157629/linking-static-libraries-to-other-static-libraries
https://users.rust-lang.org/t/linking-with-custom-c-library/637/9

can we extract obj files from the final built binary?
