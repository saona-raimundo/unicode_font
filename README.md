# unicode_font

[![Download](https://img.shields.io/crates/d/unicode_font)](https://crates.io/crates/unicode_font)
[![License](https://img.shields.io/crates/l/unicode_font)](https://github.com/saona-raimundo/unicode_font)
[![Docs](https://docs.rs/unicode_font/badge.svg)](https://docs.rs/unicode_font)
[![Crate](https://img.shields.io/crates/v/unicode_font.svg)](https://crates.io/crates/unicode_font)

Convert unicode characters between fonts.

[Playground](https://saona-raimundo.github.io/unicode_font/)

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
