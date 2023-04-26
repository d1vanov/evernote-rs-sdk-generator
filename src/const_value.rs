use thrift_parser::basic::Identifier;
use thrift_parser::constant::ConstValue;

pub fn const_value_repr<F>(c: &ConstValue, is_static: bool, get_identifier_is_static: &F) -> String
where
    F: Fn(&Identifier) -> bool,
{
    match c {
        ConstValue::Identifier(i) => format!("{}", i.as_str()),
        ConstValue::Literal(l) => {
            if is_static {
                format!("\"{}\"", l.as_str())
            } else {
                format!("String::from(\"{}\")", l.as_str())
            }
        }
        ConstValue::Double(d) => d.to_string(),
        ConstValue::Int(i) => i.to_string(),
        ConstValue::List(c) => {
            let mut list_items: Vec<String> = Default::default();
            for i in c.clone().into_inner() {
                let mut repr = const_value_repr(&i, is_static, get_identifier_is_static);
                if let ConstValue::Identifier(j) = i {
                    if !get_identifier_is_static(&j) {
                        repr = format!("{}.clone()", j.as_str());
                    }
                }
                list_items.push(repr);
            }
            format!("vec![{}]", list_items.join(", "))
        }
        ConstValue::Map(m) => {
            let mut map_items: Vec<String> = Default::default();
            for (i, j) in m.clone().into_inner() {
                map_items.push(format!(
                    "({}, {})",
                    const_value_repr(&i, is_static, get_identifier_is_static),
                    const_value_repr(&j, is_static, get_identifier_is_static)
                ));
            }
            format!("HashMap::from({})", map_items.join(", "))
        }
    }
}
