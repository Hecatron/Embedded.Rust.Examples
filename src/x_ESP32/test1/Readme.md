# Readme

This is a test example of using the ESP IDF with Rust


## lib-idf

This is a build of a dummy project using the esp idf build system
Using python, cmake, ninja
The goal here is to build what we need from the IDF into a library file that we can link against


## lib-idf-rust

This is a rust library that contains a generated wrapper for accessing the idf library


## TODO

Next we need to setup a dummy project
Add a library reference to lib-idf-rust
then link in the objects needed from lib-idf

Also need to see if we can link those same objects from within lib-idf-rust
see how it affects the space useage

