use std::fmt::{Debug, Display, Formatter};
use regex::Regex;

extern crate strum;
extern crate strum_macros;

use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug,PartialEq)]
pub enum Token {
    Identifier(String),
    Reserve(ReservedWord),
    Punctuation(PunctuationToken),
    Numeric,
    Literal(String),
}

// const IDENTIFIER_REGEX: Regex = Regex::new(r#"[\w][\w\d_]*"#).unwrap();

impl Token {

    pub fn parse_token(lexeme: &str) -> Result<Token, std::io::Error> {
        let mut opt: Option<ReservedWord> = ReservedWord::from_str(lexeme);
        if opt.is_some() {
           return Ok(Token::Reserve(opt.unwrap()));
        }

        let IDENTIFIER_REGEX: Regex = Regex::new(r#"[\w][\w\d_]*"#).unwrap();
        if IDENTIFIER_REGEX.is_match(lexeme) {
            return Ok(Token::Identifier(String::from(lexeme)));
        }

        let mut opt: Option<PunctuationToken> = PunctuationToken::from_str(lexeme);
        if opt.is_some() {
            return Ok(Token::Punctuation(opt.unwrap()));
        }

        Ok(Token::Numeric)
    }

}

/// Reserved word
#[derive(EnumIter, PartialEq)]
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

// TODO Replaced by macro in the future
impl ReservedWord {
    pub fn from_str(name: &str) -> Option<ReservedWord> {
        return match name {
            "as"        =>  Some(ReservedWord::As        ),
            "async"     =>  Some(ReservedWord::Async     ),
            "await"     =>  Some(ReservedWord::Await     ),
            "break"     =>  Some(ReservedWord::Break     ),
            "const"     =>  Some(ReservedWord::Const     ),
            "continue"  =>  Some(ReservedWord::Continue  ),
            "else"      =>  Some(ReservedWord::Else      ),
            "enum"      =>  Some(ReservedWord::Enum      ),
            "false"     =>  Some(ReservedWord::False     ),
            "func"      =>  Some(ReservedWord::Func      ),
            "for"       =>  Some(ReservedWord::For       ),
            "if"        =>  Some(ReservedWord::If        ),
            "impl"      =>  Some(ReservedWord::Impl      ),
            "in"        =>  Some(ReservedWord::In        ),
            "let"       =>  Some(ReservedWord::Let       ),
            "loop"      =>  Some(ReservedWord::Loop      ),
            "module"    =>  Some(ReservedWord::Module    ),
            "public"    =>  Some(ReservedWord::Public    ),
            "ref"       =>  Some(ReservedWord::Ref       ),
            "return"    =>  Some(ReservedWord::Return    ),
            "static"    =>  Some(ReservedWord::Static    ),
            "struct"    =>  Some(ReservedWord::Struct    ),
            "super"     =>  Some(ReservedWord::Super     ),
            "true"      =>  Some(ReservedWord::True      ),
            "type"      =>  Some(ReservedWord::Type      ),
            "union"     =>  Some(ReservedWord::Union     ),
            "unsafe"    =>  Some(ReservedWord::Unsafe    ),
            "use"       =>  Some(ReservedWord::Use       ),
            "var"       =>  Some(ReservedWord::Var       ),
            "where"     =>  Some(ReservedWord::Where     ),
            "while"     =>  Some(ReservedWord::While     ),
            _ => None,
        }
    }
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
pub enum PunctuationToken {
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

impl PunctuationToken {
    pub fn from_str(name: &str) -> Option<PunctuationToken> {
        return match name {
            "."        => Some(PunctuationToken::Dot         ),
            ","        => Some(PunctuationToken::Comma       ),
            ";"        => Some(PunctuationToken::SemiColon   ),
            ":"        => Some(PunctuationToken::Colon       ),
            "+"        => Some(PunctuationToken::Plus        ),
            "-"        => Some(PunctuationToken::Minus       ),
            "*"        => Some(PunctuationToken::Asterisk    ),
            "/"        => Some(PunctuationToken::Slash       ),
            "%"        => Some(PunctuationToken::Percentage  ),
            "("        => Some(PunctuationToken::LParenthesis),
            ")"        => Some(PunctuationToken::RParenthesis),
            "["        => Some(PunctuationToken::LBracket    ),
            "]"        => Some(PunctuationToken::RBracket    ),
            "{"        => Some(PunctuationToken::LBrace      ),
            "}"        => Some(PunctuationToken::RBrace      ),
            "<"        => Some(PunctuationToken::Less        ),
            ">"        => Some(PunctuationToken::Greater     ),
            "<="       => Some(PunctuationToken::LessEqual   ),
            ">="       => Some(PunctuationToken::GreaterEqual),
            "=="       => Some(PunctuationToken::Equal       ),
            "!="       => Some(PunctuationToken::NotEqual    ),
            "="        => Some(PunctuationToken::Assigment   ),
            "!"        => Some(PunctuationToken::Not         ),
            "&&"       => Some(PunctuationToken::And         ),
            "||"       => Some(PunctuationToken::Or          ),
            "^"        => Some(PunctuationToken::Xor         ),
            "?"        => Some(PunctuationToken::Question    ),
            "'"        => Some(PunctuationToken::SQuote      ),
            "\""       => Some(PunctuationToken::DQuote      ),
            "\"\"\""   => Some(PunctuationToken::TQuote      ),
            "->"       => Some(PunctuationToken::Arrow       ),
            "=>"       => Some(PunctuationToken::DArrow      ),
            "::"       => Some(PunctuationToken::DColon      ),
            _ => None,
        }
    }
}

impl Display for PunctuationToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for PunctuationToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   PunctuationToken::Dot              =>      ".",
                   PunctuationToken::Comma            =>      ",",
                   PunctuationToken::SemiColon        =>      ";",
                   PunctuationToken::Colon            =>      ":",
                   PunctuationToken::Plus             =>      "+",
                   PunctuationToken::Minus            =>      "-",
                   PunctuationToken::Asterisk         =>      "*",
                   PunctuationToken::Slash            =>      "/",
                   PunctuationToken::Percentage       =>      "%",
                   PunctuationToken::LParenthesis     =>      "(",
                   PunctuationToken::RParenthesis     =>      ")",
                   PunctuationToken::LBracket         =>      "[",
                   PunctuationToken::RBracket         =>      "]",
                   PunctuationToken::LBrace           =>      "{",
                   PunctuationToken::RBrace           =>      "}",
                   PunctuationToken::Less             =>      "<",
                   PunctuationToken::Greater          =>      ">",
                   PunctuationToken::LessEqual        =>      "<=",
                   PunctuationToken::GreaterEqual     =>      ">=",
                   PunctuationToken::Equal            =>      "==",
                   PunctuationToken::NotEqual         =>      "!=",
                   PunctuationToken::Assigment        =>      "=",
                   PunctuationToken::Not              =>      "!",
                   PunctuationToken::And              =>      "&&",
                   PunctuationToken::Or               =>      "||",
                   PunctuationToken::Xor              =>      "^",
                   PunctuationToken::Question         =>      "?",
                   PunctuationToken::SQuote           =>      "'",
                   PunctuationToken::DQuote           =>      "\"",
                   PunctuationToken::TQuote           =>      "\"\"\"",
                   PunctuationToken::Arrow            =>      "->",
                   PunctuationToken::DArrow           =>      "=>",
                   PunctuationToken::DColon           =>      "::",
               }
        )
    }
}

///
/// Token stream
///
#[derive(Debug)]
pub struct TokenStream {
    token: Vec<Token>,
}

impl TokenStream {

    pub fn new() -> TokenStream {
        TokenStream {
            token: Vec::new()
        }
    }

    pub fn put(&mut self, token: Token) {
        &self.token.push(token);
    }

}