use std::process;

use evernote_rs_sdk_generator::Config;

fn main() {
    let config = Config::from_clap_app().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = evernote_rs_sdk_generator::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
