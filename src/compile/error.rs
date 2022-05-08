use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct LexicalError {
    msg: String,
}

impl Error for LexicalError {
}

impl LexicalError {
    pub fn from_str(msg: &str) -> Self {
        Self {
            msg: msg.to_string()
        }
    }
}

impl LexicalError {
    pub fn from(msg: Box<dyn ToString>) -> Self {
        Self {
            msg: msg.to_string()
        }
    }
}

#[allow(unused_variables)]
impl Debug for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(unused_variables)]
impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct SyntaxError {
}

impl Error for SyntaxError {
}

#[allow(unused_variables)]
impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(unused_variables)]
impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct SemanticError {
}

impl Error for SemanticError {
}

#[allow(unused_variables)]
impl Debug for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(unused_variables)]
impl Display for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}