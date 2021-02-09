use std::{
  //env,
  error::Error,
  //ffi::OsStr,
  //fs::read_to_string,
  //io::{BufReader, BufRead, Write},
  //os::unix::ffi::OsStrExt,
  //path::PathBuf,
  //process::{Command, Stdio},
};

//use bindgen::EnumVariation;

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=src/bindings.h");
  println!("cargo:rerun-if-changed=src/sdkconfig.h");

  Ok(())
}
