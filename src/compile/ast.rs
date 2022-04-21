use super::token::Token;
pub(super) struct ASTParser {}

impl ASTParser {
    pub fn new() -> ASTParser {
        ASTParser {}
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Vec<ASTNode> {
        Vec::default()
    }
}

pub(super) struct ASTNode {}
