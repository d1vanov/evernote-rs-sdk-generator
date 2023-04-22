// use thrift_parser::basic::Identifier;
use thrift_parser::types::FieldType;

pub fn field_type_name(f: &FieldType) -> String {
    match f {
        // FIXME: find out how to extract string from identifier
        // Identifier(s) => s,
        FieldType::Bool => String::from("bool"),
        FieldType::Byte => String::from("u8"),
        FieldType::I8 => String::from("i8"),
        FieldType::I16 => String::from("i16"),
        FieldType::I32 => String::from("i32"),
        FieldType::I64 => String::from("i64"),
        FieldType::Double => String::from("f64"),
        FieldType::String => String::from("String"),
        FieldType::Binary => String::from("Vec<u8>"),
        // FIXME: find out some way to convert these to strings too
        /*
        Map(Box(from), Box(to)) => format!(
            "Map<Box<{}>, Box<{}>>",
            field_type_name(from),
            field_type_name(to)
        ),
        Set(Box(f)) => format!("Set<Box<{}>>", field_type_name(f)),
        List(Box(f)) => format!("List<Box<{}>>", field_type_name(f)),
        */
        _ => String::from(""),
    }
}
