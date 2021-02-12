use std::{
    env,
    path::PathBuf,
};

pub struct BindingSettings {
    pub idf_path: Option<PathBuf>,
    pub target: Option<String>,
    pub linker: Option<String>,
    pub sysroot: Option<PathBuf>,
}

impl BindingSettings {

    pub fn new() -> BindingSettings {
        BindingSettings {
            idf_path: None,
            target: None,
            linker: None,
            sysroot: None,
        }
    }

    /// Read in the Settings
    pub fn read(&mut self) {
        self.get_idf_path();
        self.get_target();
        self.get_linker();
        self.get_sysroot();
    }

    /// Get the idf path from the env IDF_PATH
    fn get_idf_path(&mut self) {
        let idf_path = PathBuf::from(env::var("IDF_PATH").expect("IDF_PATH not set"));
        self.idf_path = Some(idf_path);
        let pth = self.idf_path.as_deref().unwrap();
        debug!("idf_path: {:?}", pth);
    }

    /// Get the device target - esp32 / esp8266
    fn get_target(&mut self) {
        let idf_target = env::var("TARGET").expect("TARGET not set");
        self.target = Some(idf_target);
        let tgt = self.target.as_deref().unwrap();
        debug!("idf_target: {:?}", tgt);
    }

    /// Get the targets linker
    fn get_linker(&mut self) {
        match self.target.as_deref() {
            Some("xtensa-esp32-none-elf") => { 
                let linker = env::var("RUSTC_LINKER")
                    .unwrap_or("xtensa-esp32-elf-ld".to_string());
                self.linker = Some(linker);
            },
            Some("xtensa-esp8266-none-elf") => { 
                let linker = env::var("RUSTC_LINKER")
                    .unwrap_or("xtensa-lx106-elf-ld".to_string());
                self.linker = Some(linker);
            }
            _ => {
                let tgt = self.target.as_deref().unwrap();
                warn!("Generating ESP IDF bindings for target '{}' it not supported.", tgt);
            }
        }
        if self.linker != None {
            let linker = self.linker.as_deref().unwrap();
            debug!("idf_linker: {}", linker);
        } else {
            debug!("idf_linker: None",);
        }
    }


    /// Get the sysroot from the linker
    fn get_sysroot() -> PathBuf { 
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


}
