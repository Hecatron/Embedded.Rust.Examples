# Readme

This rust library represents the bindings to the esp-idf

```
cargo install bindgen
```

## TODO

  * First we need the path to libclang.dll setting within LIBCLANG_PATH
    Currently I'm using the default mainline install of clang installed under
    "C:\Program Files\LLVM\bin"
    This needs to be added to the list of things to install

  * Next clangflags += ["-target", "xtensa"] needs to be commented out
    since that target isn't in the mainline clang
    I don't think we need it anyway.

  * Another thing is the way python is passing in the cmd line parameters on execute
    passing in everything directly with spaces at the command line prompt seems to nearly work
    but there's some kind of issue with the method I'm using to pass in the parameters, maybe the clang params should be grouped by space



  * sys/reent.h not found when run directly
    I think I need to add the include -IC:\Apps\xtensa-esp32-elf\xtensa-esp32-elf\sys-include

  * next missing include is sdkconfig.h, this seems to be a generated file from the project
    -ID:\SourceCode\Local\Hecatron\Hecatron.Local\Embedded.Rust.Examples\src\ESP32\test1\lib-idf\build\config

  * next is hal/cpu_ll.h
    -ID:\SourceCode\External\esp-idf\components\soc\src\esp32\include

  * next is soc/cpu_caps.h
    -ID:\SourceCode\External\esp-idf\components\soc\soc\esp32\include


I think that's everything, it seems to generate a file with the above
Look at building the lib-idf project with idf.py next to see how that turns out
we may be able to get rid of some of the above includes

We also need to do a rust fmt on the output after it's generated
