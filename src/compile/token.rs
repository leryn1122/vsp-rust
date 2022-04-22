use std::fmt::{Debug, Display, Formatter};
//
// ///
// /// Enumeration for token accepted by scanner.
// ///
// // #[derive(Debug)]
// pub enum Token {
//     // Bool(bool),
//     // Int(i64),
//     // Float(f64),
//
//     // Statement
//     /**   .   **/  Dot,
//     /**   ,   **/  Comma,
//     /**   ;   **/  SemiColon,
//
//     //
//     /**   :   **/  Colon,
//     /**   +   **/  Plus,
//     /**   -   **/  Minus,
//     /**   *   **/  Asterisk,
//     /**   /   **/  Slash,
//     /**   %   **/  Percentage,
//
//     //
//     /**   (   **/  LParenthesis,
//     /**   )   **/  RParenthesis,
//     /**   [   **/  LBracket,
//     /**   ]   **/  RBracket,
//     /**   {   **/  LBrace,
//     /**   }   **/  RBrace,
//
//     // Comparison
//     /**   <   **/  Less,
//     /**   >   **/  Greater,
//     /**   <=  **/  LessEqual,
//     /**   >=  **/  GreaterEqual,
//     /**   ==  **/  Equal,
//     /**   !=  **/  NotEqual,
//     /**   =   **/  Assigment,
//
//     // Logic Operators
//     /**   !   **/  Not,
//     /**   &&  **/  And,
//     /**   ||  **/  Or,
//     /**   ^   **/  Xor,
//
//     // Bit Operators
//     // TODO
//
//     // Quotation
//     /**   ?   **/  Question,
//     /**   '   **/  SQuote,
//     /**   \   **/  DQuote,
//     /**   \   **/  TQuote,
//
//     // Lambda
//     /**   ->  **/  Arrow,
//     /**   =>  **/  DArrow,
//     /**   ::  **/  DColon,
// }
//
// impl Display for Token {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
//
// impl Debug for Token {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}",
//             match self {
//                 Token::Dot              =>      ".",
//                 Token::Comma            =>      ",",
//                 Token::SemiColon        =>      ";",
//                 Token::Colon            =>      ":",
//                 Token::Plus             =>      "+",
//                 Token::Minus            =>      "-",
//                 Token::Asterisk         =>      "*",
//                 Token::Slash            =>      "/",
//                 Token::Percentage       =>      "%",
//                 Token::LParenthesis     =>      "(",
//                 Token::RParenthesis     =>      ")",
//                 Token::LBracket         =>      "[",
//                 Token::RBracket         =>      "]",
//                 Token::LBrace           =>      "{",
//                 Token::RBrace           =>      "}",
//                 Token::Less             =>      "<",
//                 Token::Greater          =>      ">",
//                 Token::LessEqual        =>      "<=",
//                 Token::GreaterEqual     =>      ">=",
//                 Token::Equal            =>      "==",
//                 Token::NotEqual         =>      "!=",
//                 Token::Assigment        =>      "=",
//                 Token::Not              =>      "!",
//                 Token::And              =>      "&&",
//                 Token::Or               =>      "||",
//                 Token::Xor              =>      "^",
//                 Token::Question         =>      "?",
//                 Token::SQuote           =>      "'",
//                 Token::DQuote           =>      "\"",
//                 Token::TQuote           =>      "\"\"\"",
//                 Token::Arrow            =>      "->",
//                 Token::DArrow           =>      "=>",
//                 Token::DColon           =>      "::",
//             }
//         )
//     }
// }

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