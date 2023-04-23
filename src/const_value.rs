use thrift_parser::constant::ConstValue;

pub fn const_value_repr(c: &ConstValue) -> String {
    match c {
        ConstValue::Identifier(i) => String::from(i.as_str()),
        ConstValue::Literal(l) => String::from(l.as_str()),
        ConstValue::Double(d) => d.to_string(),
        ConstValue::Int(i) => i.to_string(),
        ConstValue::List(c) => {
            let mut list_items: Vec<String> = Default::default();
            for i in c.clone().into_inner() {
                list_items.push(const_value_repr(&i));
            }
            format!("vec![{}]", list_items.join(", "))
        }
        ConstValue::Map(m) => {
            let mut map_items: Vec<String> = Default::default();
            for (i, j) in m.clone().into_inner() {
                map_items.push(format!(
                    "({}, {})",
                    const_value_repr(&i),
                    const_value_repr(&j)
                ));
            }
            format!("std::collections::HashMap::from({})", map_items.join(", "))
        }
    }
}
