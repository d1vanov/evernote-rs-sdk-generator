use std::{error::Error, fs, path::PathBuf};

use clap::{App, Arg};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_dir = PathBuf::from(config.input_thrift_idl_files_folder);
    if !input_dir.is_dir() {
        Err("Input path is not a directory!")?;
    }

    let output_dir = PathBuf::from(config.output_sdk_code_folder);
    if !output_dir.is_dir() {
        Err("Output path is not a directory!")?;
    }

    let output_dir_permissions = fs::metadata(output_dir)?.permissions();
    if output_dir_permissions.readonly() {
        Err("Output dir is read-only!")?;
    }

    let mut thrift_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let entry_name = match entry.file_name().into_string() {
            Ok(e) => e,
            Err(_) => {
                return Err(format!(
                    "Detected file name in input dir which is not a valid Unicode!"
                ))?
            }
        };

        if !entry_name.ends_with(".thrift") {
            continue;
        }

        thrift_files.push(entry.path());
    }

    // TODO: implement further

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub input_thrift_idl_files_folder: String,
    pub output_sdk_code_folder: String,
}

impl Config {
    pub fn from_clap_app() -> Result<Config, &'static str> {
        let matches = App::new("Evernote Rust SDK generator")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Dmitry Ivanov <dm.vl.ivanov@gmail.com>")
            .about("Tool generating source code for Evernote Rust SDK")
            .arg(
                Arg::with_name("input")
                    .help("Input directory with Evernote SDK thrift IDL files")
                    .short("i")
                    .long("input")
                    .required(true),
            )
            .arg(
                Arg::with_name("output")
                    .help("Output directory for generated Evernote Rust SDK source code")
                    .short("o")
                    .long("output")
                    .required(true),
            )
            .get_matches();

        let input = match matches.value_of("input") {
            Some(i) => i,
            None => return Err("No input directory specified!"),
        };

        let output = match matches.value_of("output") {
            Some(o) => o,
            None => return Err("No output directory specified!"),
        };

        Ok(Config {
            input_thrift_idl_files_folder: String::from(input),
            output_sdk_code_folder: String::from(output),
        })
    }
}
