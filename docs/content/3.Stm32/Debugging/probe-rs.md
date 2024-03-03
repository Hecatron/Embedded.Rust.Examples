# Debugging

  * https://probe.rs/docs/tools/debugger/

The way Probe-Rs works is that it has it's own CLI application.
This cli app connects to the target device directly, flashes any firmware, then presents a DAP server for debugging via visual studio code.
It can also present itself as a GDB server, but I believe this is more limited in features.

## VSCode Extension

For the vscode extension, this connects to a DAP server as above, which can also be over the network.
For connecting to a GDB Server, the cortex-debug vscode extension would likley need to be used instead.
In which case you're probably better off using openocd or JLink's GDB server in that scenario.

## launch.json

For the vscode launch.json file we can use the below for debugging

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "probe-rs Start Debug",
      "type": "probe-rs-debug",
      "request": "launch",
      "flashingConfig": {
        "flashingEnabled": true,
        "haltAfterReset": false
      },
      "chip": "STM32F767ZITx",
      "coreConfigs": [
        {
          "coreIndex": 0,
          "programBinary": "target\\thumbv7em-none-eabihf\\debug\\blinky1",
          "svdFile": ".vscode\\svd\\stm32f7x7.svd.patched"
        }
      ],
      "preLaunchTask": "${defaultBuildTask}",
    }
  ]
}
```

## SVD File

As an optional extra we can pass in a path to an svd file to see the peripherals.
This can be downloaded from:

  * https://stm32-rs.github.io/stm32-rs/
