use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{field_type, thrift_entities};

pub fn generate_constants(output_dir: &str, entities: &thrift_entities::ThriftEntities) {
    let f = File::open(format!("{}/constants.rs", output_dir))
        .expect("Cannot create constants.rs file for writing");
    let mut bw = BufWriter::new(f);

    for constant in &entities.consts {
        // FIXME: need to convert Identifier and ConstValue to String
        bw.write(
            format!(
                "const {}: {} = {}",
                field_type::field_type_name(&constant.type_),
                "todo",
                "todo"
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
