use std::{
    env,
    //error::Error,
    //ffi::OsStr,
    //fs::read_to_string,
    //io::{BufReader, BufRead, Write},
    //os::unix::ffi::OsStrExt,
    //os::windows::ffi::OsStrExt,
    path::PathBuf,
    //process::{Command, Stdio},
};

use globwalk::GlobWalker;

extern crate globwalk;

pub fn get_idf_path() -> PathBuf {
    // Make sure the IDF_PATH is set in env and get the value
    let idf_path = PathBuf::from(env::var("IDF_PATH").expect("IDF_PATH not set"));
    println!("Debug: idf_path {:?}", idf_path);
    return idf_path;
}

pub fn get_sysroot() -> PathBuf {
    // Get the sysroot from the linker
    let sysroot = PathBuf::from(env::var("SYS_ROOT").expect("SYS_ROOT not set"));
    println!("Debug: sysroot {:?}", sysroot);
    /*let sysroot = Command::new(linker)
    .arg("--print-sysroot")
    .output()
    .map(|mut output| {
        // Remove newline from end.
        output.stdout.pop();
        PathBuf::from(OsStr::from_bytes(&output.stdout))
        .canonicalize().expect("failed to canonicalize sysroot")
    })
    .expect("failed getting sysroot");
    */
    return sysroot;
}

pub fn get_tgt_linker() -> (String, String) {
    // `let` can be used to bind the members of a tuple to variables
      // Get the target and linker exe
    // target = "xtensa-esp32-none-elf", pulled from TARGET env
    // linker = xtensa-esp32-elf-ld
    // Set the rustc cfg for esp32 conditional compile
    let (idf_target, linker) = match env::var("TARGET").unwrap().as_ref() {
        "xtensa-esp32-none-elf" => {
            println!(r#"cargo:rustc-cfg=target_device="esp32""#);
            ("esp32".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-esp32-elf-ld".to_string()))
        },
        "xtensa-esp8266-none-elf" => {
            println!(r#"cargo:rustc-cfg=target_device="esp8266""#);
            ("esp8266".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-lx106-elf-ld".to_string()))
        },
        target => {
            println!("cargo:warning=Generating ESP IDF bindings for target '{}' it not supported. The resulting crate will be empty.", target);
            return (String::from(""), String::from(""));
        },
    };
    println!("Debug: idf_target {:?}", idf_target);
    println!("Debug: linker {:?}", linker);

    (idf_target, linker)
}
