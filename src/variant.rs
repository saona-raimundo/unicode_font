//! Font variants.

/// Font variants we deal with.
///
/// # Main feature
///
/// Implements `FromString` to parse a variant from the UNICODE name in English.
#[derive(Debug, strum::Display, strum::EnumString, strum::EnumIter, Clone, Copy)]
pub enum Variant {
    #[strum(ascii_case_insensitive)]
    ArabicMathematical,
    #[strum(ascii_case_insensitive)]
    ArabicMathematicalInitial,
    #[strum(ascii_case_insensitive)]
    Bold,
    #[strum(ascii_case_insensitive)]
    BoldItalic,
    #[strum(ascii_case_insensitive)]
    BoldFraktur,
    #[strum(ascii_case_insensitive)]
    BoldScript,
    #[strum(ascii_case_insensitive)]
    Circled,
    #[strum(ascii_case_insensitive)]
    Comma,
    #[strum(ascii_case_insensitive)]
    DoubleStruck,
    #[strum(ascii_case_insensitive)]
    DoubleStruckItalic,
    #[strum(ascii_case_insensitive)]
    Fraktur,
    #[strum(ascii_case_insensitive)]
    FullStop,
    #[strum(ascii_case_insensitive)]
    Fullwidth,
    #[strum(ascii_case_insensitive)]
    Italic,
    #[strum(ascii_case_insensitive)]
    Looped,
    #[strum(ascii_case_insensitive)]
    Monospace,
    #[strum(ascii_case_insensitive)]
    NegativeCircled,
    #[strum(ascii_case_insensitive)]
    NegativeSquared,
    #[strum(ascii_case_insensitive)]
    Regional,
    #[strum(ascii_case_insensitive)]
    Segmented,
    /// Few other font variants.
    /// 
    /// These are special cases that are marked as font variant in the Unicode standard,
    /// but there was no clear named variant to map it to.
    #[strum(ascii_case_insensitive)]
    Other,
    #[strum(ascii_case_insensitive)]
    Parenthesized,
    #[strum(ascii_case_insensitive)]
    Plain,
    #[strum(ascii_case_insensitive)]
    SansSerifBoldItalic,
    #[strum(ascii_case_insensitive)]
    SansSerifBold,
    #[strum(ascii_case_insensitive)]
    SansSerifItalic,
    #[strum(ascii_case_insensitive)]
    SansSerif,
    #[strum(ascii_case_insensitive)]
    MathematicalScript,
    #[strum(ascii_case_insensitive)]
    Script,
    #[strum(ascii_case_insensitive)]
    SmallCapital,
    #[strum(ascii_case_insensitive)]
    Superscript,
    #[strum(ascii_case_insensitive)]
    Subscript,
    #[strum(ascii_case_insensitive)]
    Stretched,
    #[strum(ascii_case_insensitive)]
    Squared,
    #[strum(ascii_case_insensitive)]
    Tailed,
    #[strum(ascii_case_insensitive)]
    Wide,
}

#[derive(Debug)]
pub struct ParseError {
    pub name: String,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unclassified name:\n{}", self.name)
    }
}
impl std::error::Error for ParseError {}

impl Variant {
    /// Converts a Unicode name into a Variant.
    ///
    /// # Errors
    ///
    /// If the name is unclassified.
    pub fn parse_standard_name(s: &str) -> Result<Self, ParseError> {
        let s = s.to_lowercase();
        if s.contains("superscript") | s.contains("modifier") {
            return Ok(Variant::Superscript);
        }
        if s.contains("subscript") {
            return Ok(Variant::Subscript);
        }
        if s.contains("fullwidth") {
            return Ok(Variant::Fullwidth);
        }
        if s.contains("small capital") {
            return Ok(Variant::SmallCapital);
        }
        if s.contains("sans-serif bold italic") {
            return Ok(Variant::SansSerifBoldItalic);
        }
        if s.contains("sans-serif bold") {
            return Ok(Variant::SansSerifBold);
        }
        if s.contains("sans-serif italic") {
            return Ok(Variant::SansSerifItalic);
        }
        if s.contains("sans-serif") {
            return Ok(Variant::SansSerif);
        }
        if s.contains("bold italic") {
            return Ok(Variant::BoldItalic);
        }
        if s.contains("bold script") {
            return Ok(Variant::BoldScript);
        }
        if s.contains("mathematical script") {
            return Ok(Variant::MathematicalScript);
        }
        if ["planck constant", "planck constant over two pi"].contains(&s.as_str()) | s.contains("script") {
            return Ok(Variant::Script);
        }
        if s.contains("bold fraktur") {
            return Ok(Variant::BoldFraktur);
        }
        if s.contains("fraktur") | s.contains("black-letter") {
            return Ok(Variant::Fraktur);
        }
        if s.contains("double-struck italic") | s.contains("doublestruck italic") {
            return Ok(Variant::DoubleStruckItalic);
        }
        if s.contains("double-struck") | s.contains("doublestruck") {
            return Ok(Variant::DoubleStruck);
        }
        if s.contains("bold") | s.contains("heavy") {
            return Ok(Variant::Bold);
        }
        if s.contains("italic") {
            return Ok(Variant::Italic);
        }
        if s.contains("monospace") {
            return Ok(Variant::Monospace);
        }
        if s.contains("comma") {
            return Ok(Variant::Comma);
        }
        if s.contains("full stop") {
            return Ok(Variant::FullStop);
        }
        if s.contains("negative circled") {
            return Ok(Variant::NegativeCircled);
        }
        if s.contains("negative squared") {
            return Ok(Variant::NegativeSquared);
        }
        if s.contains("circled") {
            return Ok(Variant::Circled);
        }
        if s.contains("squared") {
            return Ok(Variant::Squared);
        }
        if s.contains("regional") {
            return Ok(Variant::Regional);
        }
        if ["information source", "hebrew letter alternative ayin", "hebrew letter alternative plus sign"].contains(&s.as_str())  {
            return Ok(Variant::Other);
        }
        if s.contains("wide") {
            return Ok(Variant::Wide);
        }
        if s.contains("looped") {
            return Ok(Variant::Looped);
        }
        if s.contains("stretched") {
            return Ok(Variant::Stretched);
        }
        if s.contains("tailed") {
            return Ok(Variant::Tailed);
        }
        if s.contains("arabic mathematical initial") {
            return Ok(Variant::ArabicMathematicalInitial);
        }
        if s.contains("arabic mathematical") {
            return Ok(Variant::ArabicMathematical);
        }
        if s.contains("segmented") {
            return Ok(Variant::Segmented);
        }
        if s.contains("parenthesized") {
            return Ok(Variant::Parenthesized);
        }
        if s.contains("small") {
            return Ok(Variant::SmallCapital);
        }

        Err(ParseError {
            name: s.to_string(),
        })
    }
}
