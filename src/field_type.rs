use thrift_parser::types::FieldType;

pub fn field_type_name(f: &FieldType) -> String {
    match f {
        FieldType::Identifier(s) => String::from(s.as_str()),
        FieldType::Bool => String::from("bool"),
        FieldType::Byte => String::from("u8"),
        FieldType::I8 => String::from("i8"),
        FieldType::I16 => String::from("i16"),
        FieldType::I32 => String::from("i32"),
        FieldType::I64 => String::from("i64"),
        FieldType::Double => String::from("f64"),
        FieldType::String => String::from("String"),
        FieldType::Binary => String::from("Vec<u8>"),
        FieldType::Map(from, to) => format!(
            "HashMap<{}, {}>",
            field_type_name(&*from),
            field_type_name(&*to)
        ),
        FieldType::Set(f) => format!("HashSet<{}>", field_type_name(&*f)),
        FieldType::List(f) => format!("Vec<{}>", field_type_name(&*f)),
    }
}
