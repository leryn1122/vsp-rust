use super::token::Token;

///
/// Instance of Lexer holding a Vec of Token
///
pub(crate) struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    pub fn lexer_while_scan(&mut self) -> Vec<Token> {
        Vec::default()
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
