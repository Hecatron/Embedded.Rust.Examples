use std::{
    env,
    path::PathBuf,
    process::Command,
};

pub struct Settings {
    pub idf_path: Option<PathBuf>,
    pub target: Option<String>,
    pub linker: Option<String>,
    pub sysroot: Option<PathBuf>,
}

impl Settings {

    pub fn new() -> Settings {
        Settings {
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
        info!("idf_path: {:?}", pth);
    }

    /// Get the device target - esp32 / esp8266
    fn get_target(&mut self) {
        let idf_target = env::var("TARGET").expect("TARGET not set");
        self.target = Some(idf_target);
        let tgt = self.target.as_deref().unwrap();
        info!("target: {:?}", tgt);
    }

    /// Get the targets linker
    fn get_linker(&mut self) {
        // Check if it's been set in the env variable
        if env::var("RUSTC_LINKER").is_ok() {
            let linker = env::var("RUSTC_LINKER").unwrap();
            info!("linker: {}", linker);
            self.linker = Some(linker);
            return;
        }

        // If env isn't set then try and determine linker from target
        match self.target.as_deref() {
            Some("xtensa-esp32-none-elf") => { 
                let linker = "xtensa-esp32-elf-ld".to_string();
                self.linker = Some(linker);
            },
            Some("xtensa-esp8266-none-elf") => { 
                let linker = "xtensa-lx106-elf-ld".to_string();
                self.linker = Some(linker);
            }
            _ => {
                let tgt = self.target.as_deref().unwrap();
                warn!("Unable to determine linker for target '{}', target is unsupported.", tgt);
            }
        }
        if self.linker != None {
            let linker = self.linker.as_deref().unwrap();
            info!("linker: {}", linker);
        } else {
            info!("linker: None",);
        }
    }

    /// Get the sysroot from the linker
    fn get_sysroot(&mut self) {
        // Check if it's been set in the env variable
        if env::var("SYS_ROOT").is_ok() {
            let sysroot = env::var("SYS_ROOT").unwrap();
            info!("sysroot: {}", sysroot);
            self.sysroot = Some(PathBuf::from(sysroot));
            return;
        }

        // If not then get via the linker command
        let sysroot = Command::new(self.linker.as_deref().unwrap())
        .arg("--print-sysroot")
        .output()        
        .map(|output| {
            // Remove newline from end.
            let mut stdout = String::from_utf8(output.stdout).unwrap();
            stdout = stdout.trim_end().to_string();
            // Covert to PathBuf
            dunce::canonicalize(PathBuf::from(stdout))
            .expect("failed to strip path")
        })
        .expect("failed getting sysroot");

        self.sysroot = Some(sysroot);
        info!("sysroot: {:?}", self.sysroot.as_deref().unwrap());
        
    }
}
