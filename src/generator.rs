use std::error::Error;

use crate::{constants, enums, thrift_entities};

pub fn generate(
    output_dir: &str,
    entities: &thrift_entities::ThriftEntities,
) -> Result<(), Box<dyn Error>> {
    constants::generate_constants(output_dir, entities)?;
    enums::generate_enums(output_dir, entities)?;
    Ok(())
}
