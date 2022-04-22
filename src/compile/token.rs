use std::fmt::{Debug, Display, Formatter};

pub trait Token {

}

///
/// Token stream
///
pub struct TokenStream {
    token: Vec<dyn Token>,
}

impl TokenStream {

    pub fn put(&self, token: Box<dyn Token>) {
        &self.token.add(token);
    }

}