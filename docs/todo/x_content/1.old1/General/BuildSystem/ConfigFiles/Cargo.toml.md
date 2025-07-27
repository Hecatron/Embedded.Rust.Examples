# Cargo Config.toml

  * https://github.com/Rahix/avr-hal
  * https://doc.rust-lang.org/cargo/reference/manifest.html

Config.toml is similar to a Makefile in that it states these things are available to be built and how to build them
.config/config.toml on the other hand specifies which things to build for which targets from Config.toml

## package

  * First there is the usual metadata such as
    name / author / version / description
    documentation / readme / homepage / repository
    license / license-file / keywords / catagories

  * edition - specify a version of rust to use
  * workspace - specify the parent workspace unless this is top level
  * build - defaults to "build.rs" used as a build script
    https://doc.rust-lang.org/cargo/reference/build-scripts.html
    can be used as a script called before the project build
  * links - link to a native library
  * exclude / include - used for including source I think
    also used to track if the project should be rebuilt
  * publish - prevent or allow publishing
  * metadatable - Can be used to store anything non rust specific
  * default-run - which .rs file to use as a runable exe with main

## build target

There's several ways of defining a build target

  * [lib], [[bin]], [[example]], [[test]], [[bench]]

[[bin]] should be used for a firmware file
