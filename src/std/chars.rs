
/// Character Utils


pub fn is_alphabetic(u: u8) -> bool {
    // Uppercase: 65 ~ 90
    (u > 64 && u < 91)
    // Lowercase: 97 ~ 122
    || (u > 96 && u < 123)
    // Number digits: 48 ~ 57
    || (u > 48 && u < 58)
}