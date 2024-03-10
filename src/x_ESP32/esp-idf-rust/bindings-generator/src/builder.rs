use std::{
    //env,
    //error::Error,
    //ffi::OsStr,
    fs::read_to_string,
    io::{BufRead, BufReader, Write},
    //os::unix::ffi::OsStrExt,
    //os::windows::ffi::OsStrExt,
    path::PathBuf,
    process::{Command, Stdio},
    //path::Path,
};
use std::fs::File;


//use bindgen::EnumVariation;
//use globwalk::GlobWalker;
extern crate globwalk;
use crate::settings::Settings;

pub struct Builder {
    pub settings: Settings,
    pub comp_incs: Option<Vec<PathBuf>>,
    pub comp_mkfiles: Option<Vec<PathBuf>>,
}

impl Builder {
    pub fn new(setts: Settings) -> Builder {
        Builder {
            settings: setts,
            comp_incs: None,
            comp_mkfiles: None,
        }
    }

    /// Run the builder
    pub fn run(&mut self) {
        self.get_component_includes();
        self.get_component_mkfiles();
        self.build1();
        // TODO
    }

    /// Get component includes
    fn get_component_includes(&mut self) {
        let comp_incs: Vec<PathBuf> = globwalk::GlobWalkerBuilder::from_patterns(
            &self.settings.idf_path.as_deref().unwrap(),
            &["components/*/include"],
        )
        .build()
        .expect("Unable to glob the components directory")
        .filter_map(Result::ok)
        .map(|d| d.into_path())
        .collect();

        self.comp_incs = Some(comp_incs);
        info!("component includes:");
        for item in self.comp_incs.as_deref().unwrap() {
            println!("{:?}", item);
        }
    }

    // Get component mkfiles
    fn get_component_mkfiles(&mut self) {
        let mkfiles: Vec<PathBuf> = globwalk::GlobWalkerBuilder::from_patterns(
            &self.settings.idf_path.as_deref().unwrap(),
            &["components/*/component.mk"],
        )
        .build()
        .expect("Unable to glob the components directory")
        .filter_map(Result::ok)
        .map(|d| d.into_path())
        .collect();

        self.comp_mkfiles = Some(mkfiles);
        info!("component mkfiles:");
        for item in self.comp_mkfiles.as_deref().unwrap() {
            println!("{:?}", item);
        }
    }

    fn build1(&mut self) {
        let mut file = File::create("D:/Temp/13/5/Makefile").unwrap();

        for path in self.comp_mkfiles.as_deref().unwrap() {

            let component_path = path.parent().unwrap();
            let mut contents = read_to_string(&path).expect("failed reading component.mk").replace("$(info ", "$(warn ");

            // This sets a bunch of flags for make
            contents.insert_str(0, r"
                CONFIG_SYSVIEW_ENABLE :=
                CONFIG_AWS_IOT_SDK :=
                CONFIG_BT_ENABLED :=
                CONFIG_BLUEDROID_ENABLED :=
            ");
            contents.push_str("\n$(info ${COMPONENT_ADD_INCLUDEDIRS})");

            info!("component_path: {:?}", component_path);
            info!("contents: {:?}", contents);

            file.write_all(contents.as_bytes());

            // Runs make for a given component directory
            // Uses stdin to pass in the given Makefile content
            let mut child = Command::new("make")
                .current_dir(&component_path)
                .arg("-f")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .env("IDF_TARGET", self.settings.target.as_deref().unwrap())
                .env("SOC_NAME", self.settings.target.as_deref().unwrap())
                .env("COMPONENT_PATH", &component_path)
                .spawn()
                .expect("make failed");

            // Capture the process stdin / stdout
            let mut stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();
            // Feed the Makefile contents to stdin
            writeln!(stdin, "{}", contents).unwrap();

            // TODO parses the stdout from the make process
            let x1 =
            BufReader::new(stdout).lines()
            .filter_map(Result::ok)
            .map(|s| s.trim_end().to_string())
            .filter(|s| !s.is_empty())
            .flat_map(|s| {
                let s = s.split(' ');
                let s = s.map(|s| s.to_string());
                s.collect::<Vec<_>>().into_iter()
            })
            .map(move |s| path.parent().unwrap().join(s))
            .filter(|s| s.is_dir());

            // TODO
        }
    }
}

/*
    // Get Additional Includes
    // TODO I think this calls make?
    let component_additional_includes = globwalk::GlobWalkerBuilder::from_patterns(
        &idf_path,
        &["components/x/component.mk"],
        )
        .build()?
        .filter_map(Result::ok)
        .flat_map(|makefile| {

            //todo for each here

            let path = makefile.into_path();
            let component_path = path.parent().unwrap();

            let mut contents = read_to_string(&path).expect("failed reading component.mk").replace("$(info ", "$(warn ");
            // Define these variables since they affect `COMPONENT_ADD_INCLUDEDIRS`.
            contents.insert_str(0, r"
                CONFIG_SYSVIEW_ENABLE :=
                CONFIG_AWS_IOT_SDK :=
                CONFIG_BT_ENABLED :=
                CONFIG_BLUEDROID_ENABLED :=
            ");
            contents.push_str("\n$(info ${COMPONENT_ADD_INCLUDEDIRS})");

            info!("component_path: {:?}", component_path);
            info!("contents: {:?}", contents);

            let mut child = Command::new("make")
                .current_dir(&component_path)
                .arg("-f")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .env("IDF_TARGET", &idf_target)
                .env("SOC_NAME", &idf_target)
                .env("COMPONENT_PATH", &component_path)
                .spawn()
                .expect("make failed");

            let mut stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            writeln!(stdin, "{}", contents).unwrap();

            BufReader::new(stdout).lines()
                .filter_map(Result::ok)
                .map(|s| s.trim_end().to_string())
                .filter(|s| !s.is_empty())
                .flat_map(|s| {
                    let s = s.split(' ');
                    let s = s.map(|s| s.to_string());
                    s.collect::<Vec<_>>().into_iter()
                })
                .map(move |s| path.parent().unwrap().join(s))
                .filter(|s| s.is_dir())
        });




        let mut includes = component_includes.chain(component_additional_includes)
            .map(|include| format!("-I{}", include.display()))
            .collect::<Vec<_>>();

        includes.sort();
        includes.dedup();

        println!("Debug: includes");
        for item in includes {
            println!("{:?}", item);
        }
    }
}

*/

/*
  let bindings = bindgen::Builder::default()
    .use_core()
    .layout_tests(false)
    .ctypes_prefix("libc")
    .default_enum_style(EnumVariation::Rust { non_exhaustive: false } )
    .header("src/bindings.h")
    .clang_arg(format!("--sysroot={}", sysroot.display()))
    .clang_arg(format!("-I{}/include", sysroot.display()))
    .clang_arg("-Isrc")
    .clang_arg("-D__bindgen")
    .clang_args(&["-target", "xtensa"])
    .clang_args(&["-x", "c"])
    .clang_args(includes);

  eprintln!("{:?}", bindings.command_line_flags());

  let out_path = PathBuf::from(env::var("OUT_DIR")?);
  bindings.generate()
    .expect("Failed to generate bindings")
    .write_to_file(out_path.join("bindings.rs"))?;

*/

/*

todo probably dont need this

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
