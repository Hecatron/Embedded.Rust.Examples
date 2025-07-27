# Flashing with Cargo ESP Flash

Note currently there are issues using this tool under Windows
One of the workarounds used to be to run the following before hand to set DTR I think
```
# Sometimes this is needed to reset the board / put it into a state where cargo espflash will work
esptool.py -p COM4 flash_id
```

However this doesn't appear to work anymore
Typically to use cargo espflash
```
# Perform the flash
cargo espflash COM4
```

Currently under windows I'm getting an error of
```
error: unable to unlink old fallback exe: Access is denied. (os error 5)
```

One of the advantages of cargo espflash vs esptool is that it can handle passing in the bootloader for you.
Typically it also triggers a build of the source and the conversion of the elf to a binary at the same time.

To upload a release version of the code just add --release to the command line options
The default is a debug build / upload which can be useful for jtag debugging.
