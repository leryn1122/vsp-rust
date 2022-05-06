use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct LexicalError {
}

impl Error for LexicalError {
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