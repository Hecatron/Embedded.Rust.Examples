#![allow(dead_code)]

use std::{fs, path::Path};
use crate::proj_cfg_types::*;

pub struct ProjectConfig {
    target: EspBuildTarget,
    idf_version: EspIdfVersion,
}

impl ProjectConfig {

    fn read() -> ProjectConfig {

        // Defaults
        let mut cfg = ProjectConfig {
            target: EspBuildTarget::Esp32,
            idf_version: EspIdfVersion::V4_4,
        };
        
        // Search / load in the ./config.toml file
        let build_cfg_path = Path::new("./config.toml");
        if build_cfg_path.exists() {
            let foo = fs::read_to_string(build_cfg_path).unwrap();
            println!("{foo:?}");
        } else {
            cfg.target = EspBuildTarget::Esp32;
        }
        
        return cfg;
    }

    fn write() {

    }

}

pub fn test1() {
    let cfg = ProjectConfig::read();

}