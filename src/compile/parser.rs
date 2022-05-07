use crate::compile::ast::Statement;
use crate::compile::error::SyntaxError;
use crate::compile::token::{Token, TokenStream};

pub struct Parser {
    tokens: TokenStream,
    offset: usize,
}

impl Parser {

    pub fn from_token_stream(token_stream: TokenStream) -> Self {
        Self {
            tokens: token_stream,
            offset: 0,
        }
    }

    #[allow(unused_variables)]
    pub fn parse(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        let stmts: Vec<Statement> = Vec::new();

        while self.tokens.has_next() {
            let token= self.tokens.next();
            match token {
                Token::Identifier(id) => {
                    log::trace!("id = {}", id);
                }
                Token::Reserve(reserve) => {
                    log::trace!("reserve = {}", reserve);
                }
                Token::Punctuation(punctuation) => {
                    log::trace!("punctuation = {}", punctuation);
                }
                Token::Numeric(num) => {
                    log::trace!("numeric = {}", num);
                }
                Token::Literal(literal) => {
                    log::trace!("literal = {}", literal);
                }
            }



        }

        Ok(stmts)
    }


}