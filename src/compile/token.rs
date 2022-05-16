use std::fmt::Debug;
use lazy_static::lazy_static;
use regex::Regex;
use crate::compile::error::{
    LexicalError,
    LexicalErrorType,
    LexicalResult
};
use crate::fstd::pos::Position;

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

impl Token {
    pub const fn literal(&self) -> &'static str {
        use Token::*;

        match self {
            As                    => "as",
            Async                 => "async",
            Await                 => "await",
            Break                 => "break",
            Const                 => "const",
            Continue              => "continue",
            Else                  => "else",
            Enum                  => "enum",
            False                 => "false",
            Func                  => "func",
            For                   => "for",
            If                    => "if",
            Impl                  => "impl",
            Import                => "import",
            In                    => "in",
            Int                   => "int",
            Let                   => "let",
            Loop                  => "loop",
            Module                => "module",
            Public                => "public",
            Ref                   => "ref",
            Return                => "return",
            Static                => "static",
            Struct                => "struct",
            Super                 => "super",
            True                  => "true",
            Type                  => "type",
            Union                 => "union",
            Unsafe                => "unsafe",
            Use                   => "use",
            Var                   => "var",
            Where                 => "where",
            While                 => "while",
            Dot                   => ".",
            Comma                 => ",",
            SemiColon             => ";",
            Colon                 => ":",
            Plus                  => "+",
            Minus                 => "-",
            Asterisk              => "*",
            Slash                 => "/",
            Percentage            => "%",
            LParenthesis          => "(",
            RParenthesis          => ")",
            LBracket              => "[",
            RBracket              => "]",
            LBrace                => "{",
            RBrace                => "}",
            Less                  => "<",
            Greater               => ">",
            LessEqual             => "<=",
            GreaterEqual          => ">=",
            Equal                 => "==",
            NotEqual              => "!=",
            Assigment             => "=",
            Not                   => "!",
            And                   => "&&",
            Or                    => "||",
            Xor                   => "^",
            Question              => "?",
            SQuote                => "'",
            DQuote                => "\"",
            TQuote                => "\"\"\"",
            Arrow                 => "->",
            DArrow                => "=>",
            DColon                => "::",
            _ => { "Not a literal token." },
        }
    }
}

pub fn parse_fixed_lexeme_from_str(lexeme: &str) -> LexicalResult<Token> {
    return match lexeme {
        "as"           => Ok(Token::As          ),
        "async"        => Ok(Token::Async       ),
        "await"        => Ok(Token::Await       ),
        "break"        => Ok(Token::Break       ),
        "const"        => Ok(Token::Const       ),
        "continue"     => Ok(Token::Continue    ),
        "else"         => Ok(Token::Else        ),
        "enum"         => Ok(Token::Enum        ),
        "false"        => Ok(Token::False       ),
        "func"         => Ok(Token::Func        ),
        "for"          => Ok(Token::For         ),
        "if"           => Ok(Token::If          ),
        "impl"         => Ok(Token::Impl        ),
        "import"       => Ok(Token::Import      ),
        "in"           => Ok(Token::In          ),
        "int"          => Ok(Token::Int         ),
        "let"          => Ok(Token::Let         ),
        "loop"         => Ok(Token::Loop        ),
        "module"       => Ok(Token::Module      ),
        "public"       => Ok(Token::Public      ),
        "ref"          => Ok(Token::Ref         ),
        "return"       => Ok(Token::Return      ),
        "static"       => Ok(Token::Static      ),
        "struct"       => Ok(Token::Struct      ),
        "super"        => Ok(Token::Super       ),
        "true"         => Ok(Token::True        ),
        "type"         => Ok(Token::Type        ),
        "union"        => Ok(Token::Union       ),
        "unsafe"       => Ok(Token::Unsafe      ),
        "use"          => Ok(Token::Use         ),
        "var"          => Ok(Token::Var         ),
        "where"        => Ok(Token::Where       ),
        "while"        => Ok(Token::While       ),

        "."        => Ok(Token::Dot         ),
        ","        => Ok(Token::Comma       ),
        ";"        => Ok(Token::SemiColon   ),
        ":"        => Ok(Token::Colon       ),
        "+"        => Ok(Token::Plus        ),
        "-"        => Ok(Token::Minus       ),
        "*"        => Ok(Token::Asterisk    ),
        "/"        => Ok(Token::Slash       ),
        "%"        => Ok(Token::Percentage  ),
        "("        => Ok(Token::LParenthesis),
        ")"        => Ok(Token::RParenthesis),
        "["        => Ok(Token::LBracket    ),
        "]"        => Ok(Token::RBracket    ),
        "{"        => Ok(Token::LBrace      ),
        "}"        => Ok(Token::RBrace      ),
        "<"        => Ok(Token::Less        ),
        ">"        => Ok(Token::Greater     ),
        "<="       => Ok(Token::LessEqual   ),
        ">="       => Ok(Token::GreaterEqual),
        "=="       => Ok(Token::Equal       ),
        "!="       => Ok(Token::NotEqual    ),
        "="        => Ok(Token::Assigment   ),
        "!"        => Ok(Token::Not         ),
        "&&"       => Ok(Token::And         ),
        "||"       => Ok(Token::Or          ),
        "^"        => Ok(Token::Xor         ),
        "?"        => Ok(Token::Question    ),
        "'"        => Ok(Token::SQuote      ),
        "\""       => Ok(Token::DQuote      ),
        "\"\"\""   => Ok(Token::TQuote      ),
        "->"       => Ok(Token::Arrow       ),
        "=>"       => Ok(Token::DArrow      ),
        "::"       => Ok(Token::DColon      ),
        _ => {
            let e = LexicalError::from(LexicalErrorType::UnrecognizedCharacter, Position::NONE);
            Err(e)
        },
    }
}



//============================================================================//

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r#"^[A-Za-z][A-Za-z0-9_]*$"#).unwrap();
    static ref NUMBERIC_REGEX: Regex = Regex::new(r#"^[+-]?([1-9]\d*(\.\d*)?|0\.+\d*|0?\.0+|0)$"#).unwrap();
}


pub fn parse_token(lexeme: &str) -> LexicalResult<Token> {
    let opt: LexicalResult<Token> = parse_fixed_lexeme_from_str(lexeme);
    let token: Token;
    if opt.is_ok() {
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

pub fn is_reserve_word(token: Token) -> bool {
    return vec![
          Token::As
        , Token::Async
        , Token::Await
        , Token::Break
        , Token::Const
        , Token::Continue
        , Token::Else
        , Token::Enum
        , Token::False
        , Token::Func
        , Token::For
        , Token::If
        , Token::Impl
        , Token::Import
        , Token::In
        , Token::Int
        , Token::Let
        , Token::Loop
        , Token::Module
        , Token::Public
        , Token::Ref
        , Token::Return
        , Token::Static
        , Token::Struct
        , Token::Super
        , Token::True
        , Token::Type
        , Token::Union
        , Token::Unsafe
        , Token::Use
        , Token::Var
        , Token::Where
        , Token::While
    ].contains(&token);
}