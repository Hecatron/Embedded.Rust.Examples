# Install Flash Tools

There are two utils we can use to flash to an esp32

  * esptool.py - python script installed via python pip
  * cargo-espflash - rust based tool that can be installed via cargo

## esptool.py

This is a python based tool so you'll need python installed to install and use it.
But it allows for querying / writing / clearing the flash.
```
pip install esptool
```

## Cargo espflash

The cargo espflash tool makes uploading flash files easier
since it doesn't require manually downloading bootloaders.
```
cargo install -f cargo-espflash
```
