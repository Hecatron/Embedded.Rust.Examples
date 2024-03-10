# Todo

## Launch.json

  * Look at the LLDB Openocd setup
    https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md

        {
// TODO pre launch task for openocd
// Path to lldb in settings?
// This wont work for the esp32, the binary release of clang misses including lldb

            "name": "debug: lldb",
            "type": "lldb",
            "request": "custom",
            "cwd": "${workspaceRoot}",
            "targetCreateCommands": [
                "target create ${workspaceFolder}/target/xtensa-esp32-none-elf/debug/blinky"
            ],
            "processCreateCommands": [
                "gdb-remote localhost:3333"
            ],
            "sourceLanguages": [
                "rust"
            ]
        }