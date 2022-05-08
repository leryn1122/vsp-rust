use std::fmt::Debug;
use lazy_static::lazy_static;
use regex::Regex;
use crate::compile::token::Token::CharConstant;
use crate::VspResult;

// extern crate strum;
// extern crate strum_macros;
// use strum_macros::EnumIter;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r#"^[A-Za-z][A-Za-z0-9_]*$"#).unwrap();
    static ref NUMBERIC_REGEX: Regex = Regex::new(r#"^[+-]?([1-9]\d*(\.\d*)?|0\.+\d*|0?\.0+|0)$"#).unwrap();
}


pub fn parse_token(lexeme: &str) -> VspResult<Token> {
    let opt: Option<Token> = parse_fixed_lexeme_from_str(lexeme);
    let token: Token;
    if opt.is_some() {
        return Ok(opt.unwrap());
    }
    if IDENTIFIER_REGEX.is_match(lexeme) {
        token = Token::Identifier(String::from(lexeme));
        return Ok(token);
    }
    if NUMBERIC_REGEX.is_match(lexeme) {
        token = Token::Numeric(String::from(lexeme));
        return Ok(token);
    }
    Ok(Token::Error)
}

//============================================================================//

///
/// Token
///
///
#[derive(Debug,PartialEq)]
pub enum Token {
// Identifier && Constant
//============================================================================//
    Identifier(String),
    Numeric(String),
    CharConstant(char),
    Literal(String),
    Comment(String),

// Reserve Word
//============================================================================//

    // A-G
    As, Async, Await, Break, Const, Continue, Else, Enum, False, Func, For,

    // H-N
    If, Impl, Import, In, Int, Let, Loop, Module,

    // O-T
    Public, Ref, Return, Static, Struct, Super, True, Type,

    // U-Z
    Union, Unsafe, Use, Var, Where, While,

// Punctuation
//============================================================================//

    // Separator
    Dot                  ,/*   .   */
    Comma                ,/*   ,   */
    SemiColon            ,/*   ;   */

    //
    Colon                ,/*   :   */
    Plus                 ,/*   +   */
    Minus                ,/*   -   */
    Asterisk             ,/*   *   */
    Slash                ,/*   /   */
    Percentage           ,/*   %   */

    //
    LParenthesis         ,/*   (   */
    RParenthesis         ,/*   )   */
    LBracket             ,/*   [   */
    RBracket             ,/*   ]   */
    LBrace               ,/*   {   */
    RBrace               ,/*   }   */

    // Comparison
    Less                 ,/*   <   */
    Greater              ,/*   >   */
    LessEqual            ,/*   <=  */
    GreaterEqual         ,/*   >=  */
    Equal                ,/*   ==  */
    NotEqual             ,/*   !=  */
    Assigment            ,/*   =   */

    // Logic Operators
    Not                  ,/*   !   */
    And                  ,/*   &&  */
    Or                   ,/*   ||  */
    Xor                  ,/*   ^   */

    // Bit Operators
    // TODO

    // Quotation
    Question             ,/*   ?   */
    SQuote               ,/*   '   */
    DQuote               ,/*   \   */
    TQuote               ,/*   \   */

    // Lambda
    Arrow                ,/*   ->  */
    DArrow               ,/*   =>  */
    DColon               ,/*   ::  */

    Error
}

pub fn parse_fixed_lexeme_from_str(lexeme: &str) -> Option<Token> {
    return match lexeme {
        "as"           => Some(Token::As          ),
        "async"        => Some(Token::Async       ),
        "await"        => Some(Token::Await       ),
        "break"        => Some(Token::Break       ),
        "const"        => Some(Token::Const       ),
        "continue"     => Some(Token::Continue    ),
        "else"         => Some(Token::Else        ),
        "enum"         => Some(Token::Enum        ),
        "false"        => Some(Token::False       ),
        "func"         => Some(Token::Func        ),
        "for"          => Some(Token::For         ),
        "if"           => Some(Token::If          ),
        "impl"         => Some(Token::Impl        ),
        "import"       => Some(Token::Import      ),
        "in"           => Some(Token::In          ),
        "int"          => Some(Token::Int         ),
        "let"          => Some(Token::Let         ),
        "loop"         => Some(Token::Loop        ),
        "module"       => Some(Token::Module      ),
        "public"       => Some(Token::Public      ),
        "ref"          => Some(Token::Ref         ),
        "return"       => Some(Token::Return      ),
        "static"       => Some(Token::Static      ),
        "struct"       => Some(Token::Struct      ),
        "super"        => Some(Token::Super       ),
        "true"         => Some(Token::True        ),
        "type"         => Some(Token::Type        ),
        "union"        => Some(Token::Union       ),
        "unsafe"       => Some(Token::Unsafe      ),
        "use"          => Some(Token::Use         ),
        "var"          => Some(Token::Var         ),
        "where"        => Some(Token::Where       ),
        "while"        => Some(Token::While       ),

        "."        => Some(Token::Dot         ),
        ","        => Some(Token::Comma       ),
        ";"        => Some(Token::SemiColon   ),
        ":"        => Some(Token::Colon       ),
        "+"        => Some(Token::Plus        ),
        "-"        => Some(Token::Minus       ),
        "*"        => Some(Token::Asterisk    ),
        "/"        => Some(Token::Slash       ),
        "%"        => Some(Token::Percentage  ),
        "("        => Some(Token::LParenthesis),
        ")"        => Some(Token::RParenthesis),
        "["        => Some(Token::LBracket    ),
        "]"        => Some(Token::RBracket    ),
        "{"        => Some(Token::LBrace      ),
        "}"        => Some(Token::RBrace      ),
        "<"        => Some(Token::Less        ),
        ">"        => Some(Token::Greater     ),
        "<="       => Some(Token::LessEqual   ),
        ">="       => Some(Token::GreaterEqual),
        "=="       => Some(Token::Equal       ),
        "!="       => Some(Token::NotEqual    ),
        "="        => Some(Token::Assigment   ),
        "!"        => Some(Token::Not         ),
        "&&"       => Some(Token::And         ),
        "||"       => Some(Token::Or          ),
        "^"        => Some(Token::Xor         ),
        "?"        => Some(Token::Question    ),
        "'"        => Some(Token::SQuote      ),
        "\""       => Some(Token::DQuote      ),
        "\"\"\""   => Some(Token::TQuote      ),
        "->"       => Some(Token::Arrow       ),
        "=>"       => Some(Token::DArrow      ),
        "::"       => Some(Token::DColon      ),
        _ => None,
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
