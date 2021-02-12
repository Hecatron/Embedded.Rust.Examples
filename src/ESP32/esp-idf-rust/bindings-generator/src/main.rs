use std::{error::Error, fs::read_to_string, io::{BufReader, BufRead, Write}, path::Path, process::{Command, Stdio}};

extern crate globwalk;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
//use bindgen::EnumVariation;

mod buildfuncs;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    info!("Starting Generation");
    info!("Reading Settings");

    // Read Settings
    let mut setts = settings::BindingSettings::new();
    setts.read();
    if setts.linker == None {
        warn!("Aborting Binding Generation");
        return Ok(());
    }


    // TODO
    
    info!("Generation Complete");
    Ok(())
}

/*
fn create_child(component_path: &Path, idf_target: &String) -> &mut Command {
    let mut child = Command::new("make")
        .current_dir(&component_path)
        .arg("-f")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env("IDF_TARGET", &idf_target)
        .env("SOC_NAME", &idf_target)
        .env("COMPONENT_PATH", &component_path);
    return child;
}
*/