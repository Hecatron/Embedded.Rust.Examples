use std::error::Error;

extern crate globwalk;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod builder;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    info!("Starting Generation");
    info!("Reading Settings");

    // Read Settings
    let mut setts = settings::Settings::new();
    setts.read();
    if setts.linker == None {
        warn!("Linker not specified, aborting binding generation");
        return Ok(());
    }

    // Start building the bindings
    let mut builder = builder::Builder::new(setts);
    builder.run();
    
    info!("Generation Complete");
    Ok(())
}
