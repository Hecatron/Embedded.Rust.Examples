use std::{error::Error, fs::read_to_string, io::{BufReader, BufRead, Write}, path::Path, process::{Command, Stdio}};

extern crate globwalk;
//use bindgen::EnumVariation;

mod buildfuncs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=src/sdkconfig.h");

    let (idf_target, linker) = buildfuncs::get_tgt_linker();
    let idf_path = buildfuncs::get_idf_path();
    let sysroot = buildfuncs::get_sysroot();
    buildfuncs::print_dbg_component_includes(&idf_path);


    // Get the component includes
    let component_includes =
    globwalk::GlobWalkerBuilder::from_patterns(
    &idf_path,
    &["components/*/include"],
    )
    .build()?
    .filter_map(Result::ok)
    .map(|d| d.into_path());


    // Get Additional Includes
    // TODO I think this calls make?
    let component_additional_includes = globwalk::GlobWalkerBuilder::from_patterns(
    &idf_path,
    &["components/*/component.mk"],
    )
    .build()?
    .filter_map(Result::ok)
    .flat_map(|makefile| {

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

        println!("Debug: component_path: {:?}", component_path);
        println!("Debug: contents: {:?}", contents);

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