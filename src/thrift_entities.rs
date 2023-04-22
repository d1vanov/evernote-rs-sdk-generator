pub use thrift_parser::{
    definition::{Const, Enum, Exception, Struct, Typedef},
    header::Include,
};

#[derive(Default)]
pub struct ThriftEntities {
    pub consts: Vec<Const>,
    pub structs: Vec<Struct>,
    pub exceptions: Vec<Exception>,
    pub enums: Vec<Enum>,
    pub includes: Vec<Include>,
    pub primitive_type_typedefs: Vec<Typedef>,
    pub string_typedefs: Vec<Typedef>,
    pub byte_array_typedefs: Vec<Typedef>,
}
