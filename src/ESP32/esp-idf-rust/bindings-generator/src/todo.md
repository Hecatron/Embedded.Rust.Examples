# Todo

## Detect file changes

Not sure if we need these

``` rust
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=src/sdkconfig.h");
```

## Set target for library

``` rust
    println!(r#"cargo:rustc-cfg=target_device="esp32""#);
    println!(r#"cargo:rustc-cfg=target_device="esp8266""#);
```

## Remaining code

``` rust
  let bindings = bindgen::Builder::default()
    .use_core()
    .layout_tests(false)
    .ctypes_prefix("libc")
    .default_enum_style(EnumVariation::Rust { non_exhaustive: false } )
    .header("src/bindings.h")
    .clang_arg(format!("--sysroot={}", sysroot.display()))
    .clang_arg(format!("-I{}/include", sysroot.display()))
    .clang_arg("-Isrc")
    .clang_arg("-D__bindgen")
    .clang_args(&["-target", "xtensa"])
    .clang_args(&["-x", "c"])
    .clang_args(includes);

  eprintln!("{:?}", bindings.command_line_flags());

  let out_path = PathBuf::from(env::var("OUT_DIR")?);
  bindings.generate()
    .expect("Failed to generate bindings")
    .write_to_file(out_path.join("bindings.rs"))?;
```
