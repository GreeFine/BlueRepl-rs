extern crate exitcode;

#[macro_use]
extern crate lazy_static;

mod bluetooth_numbers;
mod controllers;
mod preset;
mod repl;
mod utils;

use clap::Parser;

use controllers::btleplug;
use preset::Preset;
use repl::Repl;
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the preset file to load
    preset_file: Option<std::path::PathBuf>,

    #[clap(short, default_value = "btleplug")]
    /// Ble lib to use :
    /// - btleplug
    /// - simpleble
    /// - bleuio
    ble_lib: String,

    /// Override preset 'autoconnect' value with true
    #[clap(short, long)]
    autoconnect: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("BlueREPL Version: {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    let mut bt = match args.ble_lib.as_str() {
        "btleplug" => btleplug::BtleplugController::new().await,
        "simpleble" => todo!("simpleble support is not yet implemented"),
        "bleuio" => todo!("bleuio support is not yet implemented"),
        n => panic!("Unknown controller id {}", n),
    };

    let mut repl = Repl::new(&mut bt).await;

    if args.preset_file != None {
        let mut pr = Preset::new(args.preset_file.unwrap()).unwrap();

        if args.autoconnect {
            pr.device.as_mut().unwrap().autoconnect = Some(true);
        }

        repl.set_preset(pr);
    }
    repl.start().await
}
