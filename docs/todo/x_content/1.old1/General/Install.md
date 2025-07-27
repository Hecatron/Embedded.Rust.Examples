# Install

  * https://www.rust-lang.org/tools/install

The first step is to install rustup-init.exe (64bit)

  * C:\Users\ric\.rustup
    The toolchains and metadata

  * C:\Users\ric\.cargo
    The cargo home directory 

  * C:\Users\ric\.cargo\bin
    Bin directory
    This gets added to the per user PATH under HKEY_CURRENT_USER/Environment/PATH

## Updates

To update rustup for the toolchains
This also updates cargo for the libs
```
rustup update
```

  * https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html


## Command Prompts

Note there's a slight issue when running rust from the "Developer Command Prompt for VS 2019"
In that it picks up on the x32 instead of the x64 toolchain under windows
To get around this we can ether just use the normal command prompt or the one for x86_x64

  * https://github.com/rust-lang/rust/issues/43468
