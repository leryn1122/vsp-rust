
/// Pattern / mode for lexer:
///   - Trivial for normal;
///   - SingleQuoted for character;
///   - DoubleQuoted for literal string.
#[derive(PartialEq)]
pub enum LexerPattern {
    Trivial,
    SingleQuoted,
    DoubleQuoted,
}