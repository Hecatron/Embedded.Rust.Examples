# Building the ESP32 LLVM / Clang Toolchain

  * https://github.com/esp-rs/rust
  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/espressif/llvm-project


A list of steps for building out the toolchain needed.

## Build the toolchain


### Download the Source

In the below example I'm using a Windows Visual Studio 2022 Command Line
```
# First lets download the source
# The directories are just ones I typically tend to use
cd D:\SourceCode\External\esp
git clone https://github.com/esp-rs/rust

# Go into the directory
cd rust
# checkout esp
git checkout esp-1.56.0
```


### Build the Source

Next to run the build
```
# Make sure you are using python 3.8 at least
python src/bootstrap/configure.py --experimental-targets=Xtensa
python x.py build --stage 2
```

As of writing the branch used is esp-1.56.0
which seems to build / run ok


### Fix the Vendor directory

This is so that building STD with Cargo does work
For windows
```
mkdir vendor
cd vendor

# For Windows we use mklink
mklink /D rustc-std-workspace-alloc ..\library\rustc-std-workspace-alloc\
mklink /D rustc-std-workspace-core ..\library\rustc-std-workspace-core\
mklink /D rustc-std-workspace-std ..\library\rustc-std-workspace-std\
```


## Switch Toolchain 

Switch to the new compiler in Rustup:
```
# Make Rustup aware of the newly built compiler
rustup toolchain link esp D:\SourceCode\External\esp\rust\build\x86_64-pc-windows-msvc\stage2

# Switch the default across to esp
rustup default esp

# Check to make sure the list of targets includes xtensa
rustc --print target-list
```
