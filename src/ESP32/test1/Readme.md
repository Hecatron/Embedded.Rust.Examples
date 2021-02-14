# Readme

This is a test example of using the ESP IDF with Rust


## lib-idf

This is a build of a dummy project using the esp idf build system
Using python, cmake, ninja
The goal here is to build what we need from the IDF into a library file that we can link against


## lib-idf-rust

This is a rust library that contains a generated wrapper for accessing the idf library


## TODO

It looks like the idf build system generates a final binary that includes a bootloader and partition table
See if we can exclude that from the build
look at the ninja build file for the targets in there
