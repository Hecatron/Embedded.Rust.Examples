# Readme

Example for a oled display
Pins PB7 and PB8 are used for i2c

  * Monochrome-096-i2c-OLED
  * https://www.instructables.com/Monochrome-096-i2c-OLED-display-with-arduino-SSD13/

## Driver

I think the driver is a ssd1306
so trying this first

  * https://github.com/jamwaffles/ssd1306

## Power

The max power available by the regulator on the nucleo board is around 500mA for 3.3V
the mcu can take up to 300mA, leaving 200mA spare
I think this display uses up to 20mA

## TODO

  * Create an embedded graphics example

According to the I2C scanner 3C is used
The wiring output from the mbed site is a bit misleading
