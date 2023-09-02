#![allow(dead_code)]

pub enum EspBuildTarget {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32c6,
    Esp32h2,
    Esp32s2,
    Esp32s3,
}

pub struct EspBuildConf<'a> {
    arch: &'a str,
    rust_target: &'a str,
    gcc_target: &'a str,
    wokwi_board: &'a str,
}

impl EspBuildTarget {
    fn get_conf(&self) -> EspBuildConf {
        match *self {
            EspBuildTarget::Esp32 => EspBuildConf {
                arch: "xtensa",
                rust_target: "xtensa-esp32-espidf",
                gcc_target: "xtensa-esp32-elf",
                wokwi_board: "board-esp32-devkit-v1",
            },
            EspBuildTarget::Esp32c2 => EspBuildConf {
                arch: "riscv",
                rust_target: "riscv32imc-esp-espidf",
                gcc_target: "riscv32-esp-elf",
                wokwi_board: "",
            },
            EspBuildTarget::Esp32c3 => EspBuildConf {
                arch: "riscv",
                rust_target: "riscv32imc-esp-espidf",
                gcc_target: "riscv32-esp-elf",
                wokwi_board: "board-esp32-c3-devkitm-1",
            },
            EspBuildTarget::Esp32c6 => EspBuildConf {
                arch: "riscv",
                rust_target: "riscv32imac-esp-espidf",
                gcc_target: "riscv32-esp-elf",
                wokwi_board: "board-esp32-c6-devkitm-1",
            },
            EspBuildTarget::Esp32h2 => EspBuildConf {
                arch: "riscv",
                rust_target: "riscv32imac-esp-espidf",
                gcc_target: "riscv32-esp-elf",
                wokwi_board: "",
            },
            EspBuildTarget::Esp32s2 => EspBuildConf {
                arch: "xtensa",
                rust_target: "xtensa-esp32s2-espidf",
                gcc_target: "xtensa-esp32s2-elf",
                wokwi_board: "board-esp32-s2-devkitm-1",
            },
            EspBuildTarget::Esp32s3 => EspBuildConf {
                arch: "xtensa",
                rust_target: "xtensa-esp32s3-espidf",
                gcc_target: "xtensa-esp32s3-elf",
                wokwi_board: "board-esp32-s3-devkitc-1",
            },
        }
    }
}

pub enum EspIdfVersion {
    V4_4,
    V5_1,
    Master,
}
