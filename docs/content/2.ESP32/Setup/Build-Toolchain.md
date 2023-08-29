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



## TODO

```
# Make sure you are using python 3.8 at least
python src/bootstrap/configure.py --experimental-targets=Xtensa

# Configure
python src/bootstrap/configure.py ${{ matrix.LLVM_ROOT_OPTION }} --experimental-targets=Xtensa --enable-extended --tools=rustfmt --dist-compression-formats='xz'

# Build
python x.py build --stage 2
```


```
mkdir vendor
cd vendor

# For Windows we use mklink
mklink /D rustc-std-workspace-alloc ..\library\rustc-std-workspace-alloc\
mklink /D rustc-std-workspace-core ..\library\rustc-std-workspace-core\
mklink /D rustc-std-workspace-std ..\library\rustc-std-workspace-std\
```


I think the way to do this would be to first add cargo to the list of tools option
```
python src/bootstrap/configure.py --experimental-targets=Xtensa --enable-extended --tools=rustfmt,cargo --dist-compression-formats="xz"
```

Insert a build before the dist so that the tools get built
```
python x.py build --stage 2
```

Then do the dist stage
```
python x.py dist --stage 2
```

At this point the dist will fail but we have the compressed files
So as part of the powershell script, copy the content of build\x86_64-pc-windows-msvc\stage2-tools-bin
into the bin directory under the destination esp dir

build-rust-dispatch





# Prepare Build
# Added Cargo
python src/bootstrap/configure.py --experimental-targets=Xtensa --enable-extended --tools=rustfmt,cargo --dist-compression-formats="xz"

# Added this
python x.py build --exclude src/doc --stage 2 

# Build Only components - not needed
# python x.py build compiler/rustc src/tools/rustfmt library/std --stage 2

# Dist the Source
#python x.py dist src --stage 2

# Dist with x.py - this fails
python x.py dist --exclude src/doc --stage 2

# Dist the tools seperatley
python x.py dist --exclude src/doc --stage 2 rustfmt cargo







# Build with x.py - dist packages - continue on error - problem with Long path on Windows
python x.py dist --stage 2

# Build with x.py - dist packages - with cached LLVM
python x.py dist --stage 2 --exclude src/doc --llvm-skip-rebuild TRUE



# Build All
python x.py build --stage 2 --exclude src/doc

# This will error out but create the needed compressed files under build/dist
python x.py dist --stage 2 --exclude src/doc

# Dist bundle for Windows
Power shell script





          cd build/dist
          mkdir esp
          7z e rust-1.56.0-dev-x86_64-pc-windows-msvc.tar.xz
          7z x rust-1.56.0-dev-x86_64-pc-windows-msvc.tar
          pushd rust-1.56.0-dev-x86_64-pc-windows-msvc
          cp -Recurse .\rustc\bin ..\esp\
          cp -Recurse .\rustc\lib ..\esp\
          cp -Recurse .\rustc\share ..\esp\
          cp -ErrorAction SilentlyContinue -Recurse .\rust-std-x86_64-pc-windows-msvc\lib\* ..\esp\lib\
          popd
          7z e rust-src-1.56.0-dev.tar.xz
          7z x rust-src-1.56.0-dev.tar
          pushd rust-src-1.56.0-dev
          cp -ErrorAction SilentlyContinue -Recurse .\rust-src\lib\* ..\esp\lib\
          popd
          7z a esp.zip esp/

D:/rust/build/dist



To get a list of targets
rustc --print=[target-list, target-cpus, target-features]