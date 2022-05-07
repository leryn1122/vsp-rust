
/// Refer to line number.
type Line = u16;

pub enum Statement {
    Value,
    While,
    For,
    Expression,
    Return,
}


/// Indicates the expression which could be valued, such as:
///
/// ```plaintext
/// 1 + (2 * 3) / 4
/// ```
///
pub enum Expression {

}

/// Indicates the local variable
pub enum Variable {
    Nullptr,

    LocalInt(Line),
    LocalBool(Line),

    StaticInt(Line),
    StaticBool(Line),
}