
///
/// Polish Notation
///
/// ```plaintext
/// 9 - 5       ==> 9 5 -
/// 9 - (5 + 2) ==> 9 5 2 + -
/// ```
pub fn generate_ast() {

}

pub trait SyntaxNode {

    fn add_child(&mut self, child: Box<dyn SyntaxNode>);

    fn get_child(&self, ) -> &Vec<Box<dyn SyntaxNode>>;

    fn attribute(&self, ) -> Option<&String>;

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