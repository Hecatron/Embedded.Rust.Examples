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
    //path::Path,
};

//use bindgen::EnumVariation;
//use globwalk::GlobWalker;
extern crate globwalk;
use crate::settings::Settings;

pub struct Builder {
    pub settings: Option<Settings>,
}

impl Builder {

    pub fn new() -> Builder {
        Builder {
            settings: None,
        }
    }

    /// Run the builder
    pub fn run(&mut self) {

        // TODO

        //self.print_dbg_component_includes();
    }

    pub fn print_dbg_component_includes(&mut self, idf_path: &PathBuf) {

        let component_includes =
        globwalk::GlobWalkerBuilder::from_patterns(
        &idf_path,
        &["components/*/include"],
        )
        .build().unwrap()
        .filter_map(Result::ok)
        .map(|d| d.into_path());
    
        println!("Debug: component_includes");
        for item in component_includes {
            println!("{:?}", item.to_str());
        }
    }
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
