use std::io::{BufWriter, Write};
use std::{error::Error, fs::File};

use thrift_parser::types::FieldType;

use crate::{const_value, field_type, thrift_entities};

fn constants_uses(entities: &thrift_entities::ThriftEntities) -> &'static str {
    let mut has_set = false;
    let mut has_map = false;
    for constant in &entities.consts {
        match constant.type_ {
            FieldType::Set(_) => has_set = true,
            FieldType::Map(_, _) => has_map = true,
            _ => (),
        }
    }
    if has_set && has_map {
        return "use std::collections::{HashMap, HashSet};";
    } else if has_set {
        return "use std::collections::HashSet;";
    } else if has_map {
        return "use std::collections::HashMap;";
    }
    ""
}

pub fn generate_constants(
    output_dir: &str,
    entities: &thrift_entities::ThriftEntities,
) -> Result<(), Box<dyn Error>> {
    let f = File::create(format!("{}/constants.rs", output_dir))
        .expect("Cannot create constants.rs file for writing");
    let mut bw = BufWriter::new(f);

    let uses = constants_uses(entities);
    if !uses.is_empty() {
        bw.write(format!("{}\n\n", uses).as_bytes())?;
    }

    for constant in &entities.consts {
        bw.write(
            format!(
                "pub const {}: {} = {}\n",
                &constant.name.as_str(),
                field_type::field_type_name(&constant.type_),
                const_value::const_value_repr(&constant.value)
            )
            .as_bytes(),
        )?;
    }

    bw.flush().map_err(|e| e.into())
}
