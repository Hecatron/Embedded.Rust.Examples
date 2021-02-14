# Readme

Test build for the idf

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

The issue is definitley in the cmake stage
  * Setting a custom python within cmake nearly works
    but causes the parition table to fail to build under ninja
  * do we need the env var IDF_PYTHON_ENV_PATH setup ?

maybe try passing this to cmake

-DPYTHON_LIBRARY=$ANACONDA_HOME/lib/libpython2.7.so \
-DPYTHON_INCLUDE_DIR=$ANACONDA_HOME/include/python2.7/ \
-DPYTHON_EXECUTABLE=$ANACONDA_HOME/bin/python

-DPYTHON_EXECUTABLE=D:/SourceCode/Local/Hecatron/Hecatron.Local/Embedded.Rust.Examples/src/ESP32/test1/idf/build/pyenv/Scripts/python.exe






  * bring in the changes from https://github.com/lexxvir/esp32-hello
