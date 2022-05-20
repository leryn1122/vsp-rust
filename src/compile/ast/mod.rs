pub mod expr;
pub mod ns;
pub mod stmt;

/// Refer to line number.
type Line = u16;
/// Line numbers where the statement block lies.
type LineNumbers = (Line, Line);
/// Identifier in AST.
pub type Identifier = String;
///
pub type StatementBlock = crate::SmartVec<Statement>;

pub struct SyntaxNode {
    op: Identifier,
    stmt: StatementBlock,
}

impl Default for SyntaxNode {
    #[inline(always)]
    fn default() -> Self {
        Self {
            op: String::from(""),
            stmt: StatementBlock::new(),
        }
    }
}

//============================================================================//

pub enum Statement {
    Value,
    While,
    For,
    Expression,
    Return,
}

//============================================================================//

/// Indicates the expression which could be valued, such as:
///
/// ```plaintext
/// 1 + (2 * 3) / 4
/// ```
///
pub enum Expression {

}

//============================================================================//

/// Indicates the local variable
pub enum Variable {
    Nullptr,

    LocalInt(Line),
    LocalBool(Line),

    StaticInt(Line),
    StaticBool(Line),
}

//============================================================================//

struct Import {
    pub path: String,
}

impl Import {

}