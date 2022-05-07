use std::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use regex::Regex;

extern crate strum;
extern crate strum_macros;
use strum_macros::EnumIter;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r#"^[A-Za-z][A-Za-z0-9_]*"#).unwrap();
}


pub fn parse_token(lexeme: &str) -> Result<Token, std::io::Error> {
    let opt: Option<ReservedWord> = ReservedWord::from_str(lexeme);
    if opt.is_some() {
        return Ok(Token::Reserve(opt.unwrap()));
    }

    // log::trace!("lexeme = {}, result = {}", lexeme, identifier_regex.is_match(lexeme));
    if IDENTIFIER_REGEX.is_match(lexeme) {
        return Ok(Token::Identifier(String::from(lexeme)));
    }

    let opt: Option<Punctuation> = Punctuation::from_str(lexeme);
    if opt.is_some() {
        return Ok(Token::Punctuation(opt.unwrap()));
    }

    Ok(Token::Numeric(lexeme.to_string()))
}



//============================================================================//

#[derive(Debug,PartialEq)]
pub enum Token {
    Identifier(String),
    Reserve(ReservedWord),
    Punctuation(Punctuation),
    Numeric(String),
    Literal(String),
}

impl Clone for Token {
    fn clone(&self) -> Self {
        todo!()
    }
}

/// Reserved word
#[derive(EnumIter, PartialEq)]
pub enum ReservedWord {
    // A-G
    As, Async, Await, Break, Const, Continue, Else, Enum, False, Func, For,

    // H-N
    If, Impl, Import, In, Let, Loop, Module,

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
            "import"    =>  Some(ReservedWord::Import    ),
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
                   ReservedWord::Import     =>  "import",
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



//============================================================================//

///
/// Enumeration for mark / fixed token.
///
#[derive(PartialEq)]
pub enum Punctuation {
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

impl Punctuation {
    pub fn from_str(name: &str) -> Option<Punctuation> {
        return match name {
            "."        => Some(Punctuation::Dot         ),
            ","        => Some(Punctuation::Comma       ),
            ";"        => Some(Punctuation::SemiColon   ),
            ":"        => Some(Punctuation::Colon       ),
            "+"        => Some(Punctuation::Plus        ),
            "-"        => Some(Punctuation::Minus       ),
            "*"        => Some(Punctuation::Asterisk    ),
            "/"        => Some(Punctuation::Slash       ),
            "%"        => Some(Punctuation::Percentage  ),
            "("        => Some(Punctuation::LParenthesis),
            ")"        => Some(Punctuation::RParenthesis),
            "["        => Some(Punctuation::LBracket    ),
            "]"        => Some(Punctuation::RBracket    ),
            "{"        => Some(Punctuation::LBrace      ),
            "}"        => Some(Punctuation::RBrace      ),
            "<"        => Some(Punctuation::Less        ),
            ">"        => Some(Punctuation::Greater     ),
            "<="       => Some(Punctuation::LessEqual   ),
            ">="       => Some(Punctuation::GreaterEqual),
            "=="       => Some(Punctuation::Equal       ),
            "!="       => Some(Punctuation::NotEqual    ),
            "="        => Some(Punctuation::Assigment   ),
            "!"        => Some(Punctuation::Not         ),
            "&&"       => Some(Punctuation::And         ),
            "||"       => Some(Punctuation::Or          ),
            "^"        => Some(Punctuation::Xor         ),
            "?"        => Some(Punctuation::Question    ),
            "'"        => Some(Punctuation::SQuote      ),
            "\""       => Some(Punctuation::DQuote      ),
            "\"\"\""   => Some(Punctuation::TQuote      ),
            "->"       => Some(Punctuation::Arrow       ),
            "=>"       => Some(Punctuation::DArrow      ),
            "::"       => Some(Punctuation::DColon      ),
            _ => None,
        }
    }
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Punctuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Punctuation::Dot              =>      ".",
                   Punctuation::Comma            =>      ",",
                   Punctuation::SemiColon        =>      ";",
                   Punctuation::Colon            =>      ":",
                   Punctuation::Plus             =>      "+",
                   Punctuation::Minus            =>      "-",
                   Punctuation::Asterisk         =>      "*",
                   Punctuation::Slash            =>      "/",
                   Punctuation::Percentage       =>      "%",
                   Punctuation::LParenthesis     =>      "(",
                   Punctuation::RParenthesis     =>      ")",
                   Punctuation::LBracket         =>      "[",
                   Punctuation::RBracket         =>      "]",
                   Punctuation::LBrace           =>      "{",
                   Punctuation::RBrace           =>      "}",
                   Punctuation::Less             =>      "<",
                   Punctuation::Greater          =>      ">",
                   Punctuation::LessEqual        =>      "<=",
                   Punctuation::GreaterEqual     =>      ">=",
                   Punctuation::Equal            =>      "==",
                   Punctuation::NotEqual         =>      "!=",
                   Punctuation::Assigment        =>      "=",
                   Punctuation::Not              =>      "!",
                   Punctuation::And              =>      "&&",
                   Punctuation::Or               =>      "||",
                   Punctuation::Xor              =>      "^",
                   Punctuation::Question         =>      "?",
                   Punctuation::SQuote           =>      "'",
                   Punctuation::DQuote           =>      "\"",
                   Punctuation::TQuote           =>      "\"\"\"",
                   Punctuation::Arrow            =>      "->",
                   Punctuation::DArrow           =>      "=>",
                   Punctuation::DColon           =>      "::",
               }
        )
    }
}



//============================================================================//

///
/// Token stream
///
#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
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
