use std::{error::Error, fs, path::PathBuf};

use clap::{App, Arg};

use thrift_parser::{document::Document, types::FieldType, Parser};

mod const_value;
mod constants;
mod field_type;
mod generator;
mod thrift_entities;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_dir = PathBuf::from(config.input_thrift_idl_files_folder);
    if !input_dir.is_dir() {
        Err("Input path is not a directory!")?;
    }

    let output_dir = PathBuf::from(config.output_sdk_code_folder);
    if !output_dir.is_dir() {
        Err("Output path is not a directory!")?;
    }

    let output_dir_permissions = fs::metadata(&output_dir)?.permissions();
    if output_dir_permissions.readonly() {
        Err("Output dir is read-only!")?;
    }

    let output_dir_os_str = output_dir.as_os_str();
    let output_dir_str = match output_dir_os_str.to_str() {
        Some(s) => s,
        None => return Err("Cannot convert path to output dir to valid Unicode!")?,
    };

    let mut thrift_entities: thrift_entities::ThriftEntities = Default::default();
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

        // TODO: remove this when no longer necessary
        if !entry_name.starts_with("Limits") {
            continue;
        }

        let entry_content = fs::read_to_string(entry.path())?;
        let parse_result = Document::parse(&entry_content);
        let (remains, mut document) = match parse_result {
            Ok((r, d)) => (r, d),
            Err(e) => {
                return Err(format!(
                    "Cannot parse thrift file {}: {}",
                    entry_name,
                    e.to_string()
                ))?
            }
        };

        if remains.len() != 0 {
            return Err(format!(
                "Failed to fully process input thrift file {}, remains: {}",
                entry_name, remains
            ))?;
        }

        if !document.unions.is_empty() {
            return Err(format!(
                "Unions are not supported by Evernote Rust SDK generator at the moment"
            ))?;
        }

        thrift_entities.consts.append(&mut document.consts);
        thrift_entities.structs.append(&mut document.structs);
        thrift_entities.exceptions.append(&mut document.exceptions);
        thrift_entities.enums.append(&mut document.enums);
        thrift_entities.includes.append(&mut document.includes);

        for typedef in document.typedefs {
            match typedef.old {
                FieldType::String => thrift_entities.string_typedefs.push(typedef),
                FieldType::Binary => thrift_entities.byte_array_typedefs.push(typedef),
                _ => thrift_entities.primitive_type_typedefs.push(typedef),
            }
        }
    }

    generator::generate(&output_dir_str, &thrift_entities)
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
                    .required(true),
            )
            .arg(
                Arg::with_name("output")
                    .help("Output directory for generated Evernote Rust SDK source code")
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
