pub mod expr;
pub mod ns;
pub mod stmt;

/// Refer to line number.
type Line = u16;

///
/// Polish Notation
///
/// ```plaintext
/// 9 - 5       ==> 9 5 -
/// 9 - (5 + 2) ==> 9 5 2 + -
/// ```
pub fn generate_ast() {

}

//============================================================================//
pub trait SyntaxNode {

    fn add_child(&mut self, child: Box<dyn SyntaxNode>);

    fn get_child(&self) -> &Vec<Box<dyn SyntaxNode>>;

    fn attribute(&self) -> Option<&String>;

}

pub struct SyntaxNodeImpl {
    pub attribute: String,
    pub children: Vec<Box<dyn SyntaxNode>>
}

impl SyntaxNodeImpl {

    fn new() -> SyntaxNodeImpl {
        todo!()
    }
}

impl SyntaxNode for SyntaxNodeImpl {

    fn add_child(&mut self, child: Box<dyn SyntaxNode>) {
        self.children.push(child)
    }

    fn get_child(&self) -> &Vec<Box<dyn SyntaxNode>> {
        &self.children
    }

    fn attribute(&self) -> Option<&String> {
        if self.children.len() == 0 {
            Some(&self.attribute)
        } else {
            None
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