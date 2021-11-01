# Building the ESP32 LLVM / Clang Toolchain

  * https://github.com/esp-rs/rust
  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/espressif/llvm-project


A list of steps for building out the toolchain needed.

## Build the toolchain


### Download the Source

For the below I I'd recommend running this from a vanilla command line
I seemed to have issues running it from a Visual Studio 2019 Command Line myself
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

### Add Components

What I've discovered is that if you don't build cargo / rust format within the rust esp toolchain
directory, this can cause problems further down the line with not being able to run rust formatting
Or with rust-analyser within Visual Studio Code

We can't use rustup component since this is a custom toolchain, but we can use x.py
so lets build some tools
```
# Add rustfmt for reformatting code
x.py build --stage 2 rustfmt
# Fixes VSCode Issues with rust-analyser
x.py build --stage 2 cargo
```

Typically the toolchain ends up in build\x86_64-pc-windows-msvc\stage2
The tools / components end up in build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release
So lets copy the built tools across
```
xcopy /e /h /c /i build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\*.exe build\x86_64-pc-windows-msvc\stage2\bin
```

(Note x.py install doesn't work too well under windows so it's easier just to do it this way)


## Switch Toolchain 

Switch to the new compiler in Rustup:
```
# Make Rustup aware of the newly built compiler
rustup toolchain link esp D:\SourceCode\External\esp\rust\build\x86_64-pc-windows-msvc\stage2

# Switch the default across to esp (optional)
rustup default esp

# Check to make sure the list of targets includes xtensa
rustc --print target-list
```
