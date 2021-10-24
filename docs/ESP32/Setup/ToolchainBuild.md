# Building the ESP32 LLVM / Clang Toolchain

  * https://github.com/esp-rs/rust
  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/espressif/llvm-project

This is more of a breif overview of building the toolchain under windows
The link to the xtensa-rust-quickstart goes into more detail above.


## Build toolchain

From a regular command line

```
# First lets download the source
# The directories are just ones I typically tend to choose
cd D:\SourceCode\External\esp
git clone https://github.com/esp-rs/rust

# Go into the directory
cd rust
# checkout esp
git checkout esp-1.56.0


# Run the build
# Make sure you are using python 3.8 at least
python src/bootstrap/configure.py --experimental-targets=Xtensa
python x.py build --stage 2
```

As of writing the branch used is esp-1.56.0
which seems to build / run ok

## Fix the Vendor directory

This is so that building STD with Cargo does work
```
mkdir vendor
cd vendor

# For Windows
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

# Check the list of targets includes xtensa
rustc --print target-list
```

## LLVM / Clang Setup

We also need a custom version of clang for the esp32

  * https://github.com/espressif/llvm-project/releases

Download and extract the latest release for windows into
C:\Apps\xtensa-esp32-elf-clang

Make sure any existing versions of LLVM are uninstalled then add the following to the path
C:\Apps\xtensa-esp32-elf-clang\bin
