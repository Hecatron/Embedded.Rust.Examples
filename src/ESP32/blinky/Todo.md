# Todo

## Config

  * try different debuggers for launch.json - CodeLLDB / Native Debug
    So far Cortex Debug seems to work
  * look at the runner under .cargo/config
  * python wrapper to set the env variables for vscode
  * test the jlink ocd setup

## Documentation

  * Add the following for install to the docs
  openocd - https://github.com/espressif/openocd-esp32/releases/tag/v0.10.0-esp32-20201202
  * Add cargo-xbuild to the installs


## Switch from Xbuild to build

Currently there's an issue with the mem functions being duplicated between esp32-hal / compiler-builtins
So to get around this we use cargo-xbuild instead of cargo

  * https://github.com/MabezDev/xtensa-rust-quickstart/issues/37
  * Once we're on cargo rename .config/config to .config/config.toml


## Size Difference

Once we switch to cargo build-std, see if there's still a 100Kb size difference in the final binary
Cargo includes the following additional libs or it might be the additional memory functions being linked in
```
   Compiling core v0.0.0 (D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\src\rust\library\core)
   Compiling rustc-std-workspace-core v1.99.0 (D:\SourceCode\External\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\src\rust\library\rustc-std-workspace-core)
```
Size increased from 473Kb to 573Kb for release
