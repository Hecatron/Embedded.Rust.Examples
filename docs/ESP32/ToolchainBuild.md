# Building the ESP32 LLVM / Clang Toolchain

  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/MabezDev/rust-xtensa
  * https://github.com/espressif/llvm-project

This is more of a breif overview of building the toolchain under windows
The link to the xtensa-rust-quickstart goes into more detail above.


## Build toolchain

From a regular command line

```
# First lets download the source
# The directories are just ones I typically tend to choose
cd D:\SourceCode\External
git clone https://github.com/MabezDev/rust-xtensa

# Go into the directory
cd rust-xtensa

# Run the build
CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
C:\Python38\python.exe src/bootstrap/configure.py --experimental-targets=Xtensa
C:\Python38\python.exe x.py build --stage 2
```

As of writing the commit I'm using is e45940c1d4b03eb3e0761e2304c5238afd591d9f
which seems to build / run ok
