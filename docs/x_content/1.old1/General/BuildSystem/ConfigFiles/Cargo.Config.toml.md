# Cargo .cargo/config.toml

I think the file can be called .cargo/config.toml or .cargo/config
.cargo/config.toml seems to be the convention

Note xargo only seems to look for .cargo/config

  * https://doc.rust-lang.org/cargo/reference/config.html

## build

This can specify which targets to build for for a project
so if multiple targets are available within the root Cargo.toml
This can be used to specify which ones to specifically build for as part of the setup.

```
[build]
target = "xtensa-esp32-none-elf" # esp32
# target = "xtensa-esp8266-none-elf" # esp8266
# target = "xtensa-none-elf" # generic xtensa target
```

We can also override the rustc and rustdoc exe's here
```
[build]
rustc = "D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc"
rustdoc = "D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc"
```


## Cpu Target options

Here we can add rust flags and  which gdb runner to use
```
[target.xtensa-esp32-none-elf] # esp32
rustflags = [
  "-C", "link-arg=-Wl,-Tlink.x",
  "-C", "link-arg=-nostartfiles",
]
```

We can also specify which thing to run when we want to run the compiled output
In this case lanch gdb
```
[target.xtensa-esp32-none-elf] # esp32
runner = "xtensa-esp32-elf-gdb -q -x openocd.gdb"
```
