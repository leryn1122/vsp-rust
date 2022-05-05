use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub enum Primitive {
    Bool(bool),
    Char(char),
    Float(f64),
    Int(i64),
    Ref,
    None,
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Int(i) => write!(f, "{}", i),
            Primitive::Char(c) => write!(f, "'{}'", c),
            Primitive::Bool(b) => write!(f, "{}", b),
            _ => todo!("")
        }
    }
}

impl Primitive {

    pub fn to_string(&self) -> &str {
        match self {
            Primitive::Bool(b) => {
                if *b {
                    "true"
                } else {
                    "false"
                }
            },
            Primitive::Char(_) => "",
            Primitive::Float(_) => "",
            Primitive::Int(_) => "",
            _ => todo!("")
        }
    }

}