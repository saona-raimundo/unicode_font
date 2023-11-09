pub mod prelude {
    pub use crate::code_to_character;
    pub use unicode_font::Variant;
}

/// Parses a `char` from a UNICODE code.
pub fn code_to_character(s: &str) -> char {
    let n = u32::from_str_radix(s, 16).expect("Invalid hex code!");
    char::from_u32(n).expect("Hex code is not a char!")
}

