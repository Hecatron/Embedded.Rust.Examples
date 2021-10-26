# LLVM Clang Install

One of the things we need for the ESP32 is a custom version of LLVM / Clang
Download and extract the latest release

  * https://github.com/espressif/llvm-project/releases

## Windows

Make sure any existing versions of LLVM are uninstalled to avoid being picked up
For Windows typically this would be some place such as
```
C:\Apps\xtensa-esp32-elf-clang
```

Add the following to the Path
```
C:\Apps\xtensa-esp32-elf-clang\bin
```
