#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "esp32")]
fn test1() {
    // Print text to the console
    //println!("Hello World!");
    let mut testint = 0;
}

//#[cfg(all(target_arch = "xtensa", target_vendor = "esp32", target_os = "none", target_env = "elf"))]

#[cfg(feature = "esp8266")]
fn test2() {
    // Print text to the console
    //println!("Hello World!");
    let mut testint = 0;
}
