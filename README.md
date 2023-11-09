# unicode_font

Convert unicode character between fonts.

[Unicode Standard Annex #44](https://www.unicode.org/reports/tr44/tr44-32.html#Character_Decomposition_Mappings) defines Character Decomposition Mapping. 
In particular, characters are given a `<font>` tag to indicate some characters are a font variant of others.
On top of these variants, we add carefully selected variants, like superscript, subscript and squared.
This extension is included by default and can be [turned off](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature).

## Characteristics

- Standard complying
	+ We follow [Unicode Standard Anex #44](https://www.unicode.org/reports/tr44/tr44-32.html#Character_Decomposition_Mappings)
	+ Additions to this mapping can be turned off
- Database-driven
	+ Code is generated from CSV files
- Hash lookup
	+ We use perfect hash functions for lookup, generally faster than binary-search on ordered tables

## Similar projects

- [YayText](https://yaytext.com)
	- Online tool to transform to multiple unicode styles
- [sprezz keyboard](https://www.sprezzkeyboard.com/)
	- Online tool to transform to multiple unicode styles
- [Unicode Toys](https://qaz.wtf/u/)
	- Transliterate plain text to obscure characters from Unicode

## Building

We use a builder script that takes into account the folder `unidata`.
The script is the crate `builder`.
Run this crate to update `unicode_font`'s maps.

We opted for this strategy as opposed to a building script `build.rs` to speed up compilation of dependent crates.

## Resources

- [technical report on Mathematics on Unicode](http://www.unicode.org/reports/tr25/)
	+ Guide on using mathematical Unicode characters
