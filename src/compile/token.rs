use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use regex::Regex;

#[derive(Debug,PartialEq)]
pub enum Token {
    Identifier(String),
    Reserve(ReservedWord),
    Mark(MarkToken),
    Numeric,
    Literal(String),
}

// const IDENTIFIER_REGEX: Regex = Regex::new(r#"[\w][\w\d_]*"#).unwrap();

impl Token {

    pub fn get_token(lexeme: &str) -> Token {



        let IDENTIFIER_REGEX: Regex = Regex::new(r#"[\w][\w\d_]*"#).unwrap();
        if IDENTIFIER_REGEX.is_match(lexeme) {
            return Token::Identifier(String::from(lexeme));
        }
        Token::Numeric
    }

}

/// Reserved word
#[derive(PartialEq)]
pub enum ReservedWord {
    // A-G
    As, Async, Await, Break, Const, Continue, Else, Enum, False, Func, For,

    // H-N
    If, Impl, In, Let, Loop, Module,

    // O-T
    Public, Ref, Return, Static, Struct, Super, True, Type,

    // U-Z
    Union, Unsafe, Use, Var, Where, While
}

impl Display for ReservedWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for ReservedWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   ReservedWord::As         =>  "as",
                   ReservedWord::Async      =>  "async",
                   ReservedWord::Await      =>  "await",
                   ReservedWord::Break      =>  "break",
                   ReservedWord::Const      =>  "const",
                   ReservedWord::Continue   =>  "continue",
                   ReservedWord::Else       =>  "else",
                   ReservedWord::Enum       =>  "enum",
                   ReservedWord::False      =>  "false",
                   ReservedWord::Func       =>  "func",
                   ReservedWord::For        =>  "for",
                   ReservedWord::If         =>  "if",
                   ReservedWord::Impl       =>  "impl",
                   ReservedWord::In         =>  "in",
                   ReservedWord::Let        =>  "let",
                   ReservedWord::Loop       =>  "loop",
                   ReservedWord::Module     =>  "module",
                   ReservedWord::Public     =>  "public",
                   ReservedWord::Ref        =>  "ref",
                   ReservedWord::Return     =>  "return",
                   ReservedWord::Static     =>  "static",
                   ReservedWord::Struct     =>  "struct",
                   ReservedWord::Super      =>  "super",
                   ReservedWord::True       =>  "true",
                   ReservedWord::Type       =>  "type",
                   ReservedWord::Union      =>  "union",
                   ReservedWord::Unsafe     =>  "unsafe",
                   ReservedWord::Use        =>  "use",
                   ReservedWord::Var        =>  "var",
                   ReservedWord::Where      =>  "where",
                   ReservedWord::While      =>  "while",
               }
        )
    }
}

///
/// Enumeration for mark / fixed token.
///
#[derive(PartialEq)]
pub enum MarkToken {
    // Statement
    /**   .   **/  Dot,
    /**   ,   **/  Comma,
    /**   ;   **/  SemiColon,

    //
    /**   :   **/  Colon,
    /**   +   **/  Plus,
    /**   -   **/  Minus,
    /**   *   **/  Asterisk,
    /**   /   **/  Slash,
    /**   %   **/  Percentage,

    //
    /**   (   **/  LParenthesis,
    /**   )   **/  RParenthesis,
    /**   [   **/  LBracket,
    /**   ]   **/  RBracket,
    /**   {   **/  LBrace,
    /**   }   **/  RBrace,

    // Comparison
    /**   <   **/  Less,
    /**   >   **/  Greater,
    /**   <=  **/  LessEqual,
    /**   >=  **/  GreaterEqual,
    /**   ==  **/  Equal,
    /**   !=  **/  NotEqual,
    /**   =   **/  Assigment,

    // Logic Operators
    /**   !   **/  Not,
    /**   &&  **/  And,
    /**   ||  **/  Or,
    /**   ^   **/  Xor,

    // Bit Operators
    // TODO

    // Quotation
    /**   ?   **/  Question,
    /**   '   **/  SQuote,
    /**   \   **/  DQuote,
    /**   \   **/  TQuote,

    // Lambda
    /**   ->  **/  Arrow,
    /**   =>  **/  DArrow,
    /**   ::  **/  DColon,
}

impl Display for MarkToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for MarkToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   MarkToken::Dot              =>      ".",
                   MarkToken::Comma            =>      ",",
                   MarkToken::SemiColon        =>      ";",
                   MarkToken::Colon            =>      ":",
                   MarkToken::Plus             =>      "+",
                   MarkToken::Minus            =>      "-",
                   MarkToken::Asterisk         =>      "*",
                   MarkToken::Slash            =>      "/",
                   MarkToken::Percentage       =>      "%",
                   MarkToken::LParenthesis     =>      "(",
                   MarkToken::RParenthesis     =>      ")",
                   MarkToken::LBracket         =>      "[",
                   MarkToken::RBracket         =>      "]",
                   MarkToken::LBrace           =>      "{",
                   MarkToken::RBrace           =>      "}",
                   MarkToken::Less             =>      "<",
                   MarkToken::Greater          =>      ">",
                   MarkToken::LessEqual        =>      "<=",
                   MarkToken::GreaterEqual     =>      ">=",
                   MarkToken::Equal            =>      "==",
                   MarkToken::NotEqual         =>      "!=",
                   MarkToken::Assigment        =>      "=",
                   MarkToken::Not              =>      "!",
                   MarkToken::And              =>      "&&",
                   MarkToken::Or               =>      "||",
                   MarkToken::Xor              =>      "^",
                   MarkToken::Question         =>      "?",
                   MarkToken::SQuote           =>      "'",
                   MarkToken::DQuote           =>      "\"",
                   MarkToken::TQuote           =>      "\"\"\"",
                   MarkToken::Arrow            =>      "->",
                   MarkToken::DArrow           =>      "=>",
                   MarkToken::DColon           =>      "::",
               }
        )
    }
}

///
/// Token stream
///
pub struct TokenStream {
    token: Vec<Token>,
}

impl TokenStream {

    pub fn put(&mut self, token: Token) {
        &self.token.push(token);
    }

}