//! Three main kind of errors occurred in the process of compile.
//!   - LexicalError
//!   - SyntaxError
//!   - SemanticError

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::fstd::pos::Position;

pub type LexicalResult<T> = Result<T, LexicalError>;
pub type SyntaxResult<T> = Result<T, SyntaxError>;
pub type SemanticResult<T> = Result<T, SemanticError>;

/// Error encountered when the lexer dealing with the source code.
pub struct LexicalError {
    pub cause: Box<LexicalErrorType>,
    pub pos: Position,
}

impl LexicalError {
    /// Default constructor.
    pub fn from(cause: LexicalErrorType, pos: Position) -> Self {
        Self {
            cause: Box::new(cause),
            pos,
        }
    }
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

#[derive(PartialOrd, PartialEq)]
pub enum LexicalErrorType {
    UnexpectedEOF,
    UnrecognizedCharacter,
}

//============================================================================//

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

//============================================================================//

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