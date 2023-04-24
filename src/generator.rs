use std::error::Error;

use crate::{constants, thrift_entities};

pub fn generate(
    output_dir: &str,
    entities: &thrift_entities::ThriftEntities,
) -> Result<(), Box<dyn Error>> {
    constants::generate_constants(output_dir, entities)?;
    Ok(())
}
