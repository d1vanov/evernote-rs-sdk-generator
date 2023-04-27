use lazy_static::lazy_static;
use regex::Regex;

pub fn preprocess_comment_contents(c: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[^\S\r\n]?[*]").unwrap();
    }
    let lines = c.lines().collect::<Vec<_>>();
    if lines.len() < 2 {
        // Comment is either single line one using "//" pattern or part of
        // multiline comment using "/*" and "*/" pattern. In the second case
        // the single line would start with optional whitespace and asterisk
        if let Some(m) = RE.find(lines[0]) {
            return format!(" *{}", &c[m.end()..]);
        }
        return String::from(c);
    }

    // For multiline comments need to remove lines which start with " *" and
    // then contain nothing more than whitespaces
    let mut result: Vec<String> = Default::default();
    for line in &lines {
        if let Some(i) = line.find('*') {
            let rest_of_line = &line[i + 1..];
            if rest_of_line.trim().len() == 0 {
                continue;
            }
            result.push(format!(" *{}", rest_of_line));
        }
    }

    result.join("\n")
}
