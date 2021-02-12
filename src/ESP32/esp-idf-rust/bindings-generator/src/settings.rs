use std::{
    env,
    path::PathBuf,
};

pub struct BindingSettings {
    pub idf_path: Option<PathBuf>,
    pub idf_target: Option<String>,
}

impl BindingSettings {

    pub fn new() -> BindingSettings {
        BindingSettings {
            idf_path: None,
            idf_target: None
        }
    }

    /// Read in the Settings
    pub fn read(&mut self) {
        self.get_idf_path();
        self.get_target();
        self.get_linker();
    }

    /// Get the idf path from the env IDF_PATH
    fn get_idf_path(&mut self) {
        let idf_path = PathBuf::from(env::var("IDF_PATH").expect("IDF_PATH not set"));
        self.idf_path = Some(idf_path);
        info!("idf_path: {:?}", self.idf_path);
    }

    /// Get the device target - esp32 / esp8266
    fn get_target(&mut self) {
        let idf_target = env::var("TARGET").expect("TARGET not set");
        self.idf_target = Some(idf_target);
        info!("idf_target: {:?}", self.idf_target);
    }

    /// Get the targets linker
    fn get_linker(&mut self) {
        // TODO check value with select case
        match self.idf_target {

            Some(String::from("xtensa-esp32-none-elf")) => { 

            },

            Some(String::from("xtensa-esp8266-none-elf")) => { 

            }

            None => {

            }
        }

    }

    fn get_tgt_linker(&mut self) {
        // `let` can be used to bind the members of a tuple to variables
          // Get the target and linker exe
        // target = "xtensa-esp32-none-elf", pulled from TARGET env
        // linker = xtensa-esp32-elf-ld
        // Set the rustc cfg for esp32 conditional compile
        let (idf_target, linker) = match env::var("TARGET").unwrap().as_ref() {
            "xtensa-esp32-none-elf" => {
                ("esp32".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-esp32-elf-ld".to_string()))
            },
            "xtensa-esp8266-none-elf" => {
                ("esp8266".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-lx106-elf-ld".to_string()))
            },
            target => {
                println!("cargo:warning=Generating ESP IDF bindings for target '{}' it not supported. The resulting crate will be empty.", target);
                //return (String::from(""), String::from(""));
                return;
            },
        };
        println!("Debug: idf_target {:?}", idf_target);
        println!("Debug: linker {:?}", linker);
    
        //(idf_target, linker)
    }

}



