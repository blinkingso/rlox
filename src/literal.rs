use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    String(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "\"{string}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}
