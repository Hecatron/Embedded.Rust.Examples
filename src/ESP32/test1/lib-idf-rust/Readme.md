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

  * the lib.rs defines some of the std types since there's no std namespace
    but this doesn't seem to work, maybe thats why the original used the 2015 type instead of 2018


::std::os::raw::c_char		i8
::std::os::raw::c_int		i32
::std::os::raw::c_uchar		u8
::std::os::raw::c_schar		i8
::std::os::raw::c_short		i16
::std::os::raw::c_ushort	u16
::std::os::raw::c_uint		u32
::std::os::raw::c_long		i32
::std::os::raw::c_ulong		u32
::std::os::raw::c_longlong	i64
::std::os::raw::c_ulonglong	u64
