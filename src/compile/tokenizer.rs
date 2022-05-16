use crate::compile::token::Token;

///
/// Token stream
///
#[derive(Debug)]
pub struct TokenStream {
    /// Vector of parsed tokens.
    tokens: Vec<Token>,
    /// Offset on tokens.
    offset: usize,
}

impl TokenStream {

    pub fn new() -> TokenStream {
        TokenStream {
            tokens: Vec::new(),
            offset: 0,
        }
    }

    #[allow(unused_must_use)]
    pub fn put(&mut self, token: Token) {
        &self.tokens.push(token);
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn has_next(&self) -> bool {
        self.offset < self.len()
    }

    pub fn next(&mut self) -> &Token {
        let token = self.tokens.get(self.offset).unwrap();
        self.offset = self.offset + 1;
        token
    }
}

/// A wrapper pointer iterates on the token stream.
pub struct TokenIterator {
    // map: HashMap<&'static str, OnParseTokenCallback>,
}

pub type OnParseTokenCallback = dyn Fn() -> Token + 'static;