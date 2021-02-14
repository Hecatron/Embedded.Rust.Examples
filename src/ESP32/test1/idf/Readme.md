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

  * Setting a custom python within cmake nearly works
    but causes the parition table to fail to build under ninja

  * Can we use a more generic python environment than py38 for tox?

  * do we need the env var IDF_PYTHON_ENV_PATH setup ?

  * bring in the changes from https://github.com/lexxvir/esp32-hello
