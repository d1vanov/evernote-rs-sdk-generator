use std::io::{BufWriter, Write};
use std::{error::Error, fs::File};

use crate::{comment, thrift_entities};

pub fn generate_enums(
    output_dir: &str,
    entities: &thrift_entities::ThriftEntities,
) -> Result<(), Box<dyn Error>> {
    let f = File::create(format!("{}/enums.rs", output_dir))
        .expect("Cannot create enums.rs file for writing");
    let mut bw = BufWriter::new(f);

    for (i, e) in entities.enums.iter().enumerate() {
        bw.write(
            format!(
                "{}pub enum {} {{\n",
                match &e.doc_comment {
                    Some(c) => format!(
                        "/**\n{}\n */\n",
                        comment::preprocess_comment_contents(c.as_str().trim())
                    ),
                    None => String::new(),
                },
                &e.name.as_str(),
            )
            .as_bytes(),
        )?;

        for c in &e.children {
            bw.write(format!("    {}", c.name.as_str()).as_bytes())?;
            if let Some(v) = c.value {
                bw.write(format!(" = {}", v.into_inner()).as_bytes())?;
            }
            bw.write(",\n".as_bytes())?;
        }

        bw.write("}\n".as_bytes())?;

        if i < entities.enums.len() - 1 {
            bw.write("\n".as_bytes())?;
        }
    }

    bw.flush().map_err(|e| e.into())
}
