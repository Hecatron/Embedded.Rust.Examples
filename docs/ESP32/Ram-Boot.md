# Ram Boot

So far I've not managed to get booting from Ram to work
```
# Attempt ram upload
esptool.py --chip esp32 --baud 115200 --port COM4 load_ram target/xtensa-esp32-none-elf/release/blinky.bin
# Image info
xtensa-esp32-elf-objdump -h target\xtensa-esp32-none-elf\release\blinky
esptool.py --chip esp32 image_info target\xtensa-esp32-none-elf\release\blinky.bin
```
