//! Unicode font conversion.
//!
//! Characters with font variants encoded in Unicode have a best-effort chosen `plain` variant.
//! `Plain` unicodes can be converted to different font variants using this crate.
//!
//! Most of the conversions use mathematical characters.
//! See [`Variant`] for the font variants included.
//!
//! # Examples
//!
//! Transform a character to a few fonts.
//! ```
//! assert_eq!(unicode_font::try_as_bold(&'a'), Some(&'\u{1D41A}')); // 𝐚
//! assert_eq!(unicode_font::try_as_circled(&'a'), Some(&'\u{24D0}')); // ⓐ
//! assert_eq!(unicode_font::try_as_monospace(&'a'), Some(&'\u{1D68A}')); // 𝚊
//! assert_eq!(unicode_font::try_as_small_capital(&'a'), None); 
//! assert_eq!(unicode_font::try_as_small_capital(&'A'), Some(&'\u{1D00}')); // ᴀ
//! assert_eq!(unicode_font::try_as_squared(&'a'), None);
//! assert_eq!(unicode_font::try_as_squared(&'A'), Some(&'\u{1F130}')); // 🄰
//! ```
//!
//! Transform a `&str` if all characters transform.
//! ```
//! fn as_bold(s: &str) -> Option<String> {
//! 	s.chars().map(|c| unicode_font::try_as_bold(&c).cloned()).collect()
//! }
//! 
//! assert_eq!(as_bold("abc"), Some(String::from("𝐚𝐛𝐜")));
//! ```
//
// # Implementation
//
// We follow the following pattern.
// - Conversion to plain variant
// - Conversion from plain variant to font variant.
// Each convertion is a table for `char` to `char` conversion.
// Some graphemes are longer than a `char`. Then, we use fixed sized arrays.

macro_rules! modules { 
	( $( $variant: ident ), *) => { 
		$( 
			paste::paste! { 
				pub use crate::$variant::[< try_as_ $variant >];  
			}

			#[doc= stringify!(Variant of Unicode symbols.)]
			pub mod $variant {
				/// Mapping of plain characters to its variant.
				///
				/// # Warnings
				///
				/// There is no Unicode standard for fonts.
				/// This is a best-effort mapping.
				///
				/// # Remarks
				///
				/// In Unicode terms, this is a simple map since it maps `char` to `char`.
				#[cfg(not(feature = "extension"))]
				paste::paste! { 
					pub const [< $variant:upper _MAP >]: phf::Map<char, char> = include!(stringify!($variant));
				} 
				#[cfg(feature = "extension")]
				paste::paste! { 
					pub const [< $variant:upper _MAP >]: phf::Map<char, char> = include!(stringify!($variant.extension));
				}

				paste::paste! { 
					/// Returns the variant version of a character if there is any.
					pub fn [< try_as_ $variant >](s: &char) -> Option<&char> {
						// For a speed up, use the map directly
						[< $variant:upper _MAP >].get(s)
					}
				}

				#[cfg(test)]
				mod tests {
				    use super::*;
				    use crate::try_as_plain;

				    paste::paste! { 
					    #[test]
					    fn [< $variant _keys_are_plain >]() {
					    	let keys: Vec<_> = [< $variant:upper _MAP >].keys().cloned().collect();
					    	let plain: Vec<_> = keys.clone().into_iter().map(|c| crate::try_as_plain(&c).cloned().unwrap()).collect(); 
					        assert_eq!(keys, plain);
					    }
					}

					paste::paste! { 
					    #[test]
					    fn [< $variant _values_are_supported >]() {
					    	for c in [< $variant:upper _MAP >].values() {
					    		crate::try_as_plain(&c).expect(&format!("failed to transform character {} to plain.", c));
					    	}
					    }
					}

					paste::paste! { 
					    #[test]
					    fn [< roundtrip_ $variant _plain_ $variant >]() {
					    	let variant: Vec<_> = [< $variant:upper _MAP >].values().cloned().collect();
					    	let double_variant: Vec<_> = variant.clone().into_iter().map(|c| try_as_plain(&c).cloned().unwrap()).map(|c| [< try_as_ $variant:lower >](&c).cloned().unwrap()).collect(); 
					        assert_eq!(variant, double_variant);
					    }
					}
				}

			}
		)*
	} 
}

modules!(
	arabic_mathematical, 
	bold,
	bold_italic,
	bold_fraktur,
	bold_script,
	circled,
	comma,
	double_struck,
	fraktur,
	full_stop,
	fullwidth,
	italic,
	looped,
	monospace,
	negative_circled,
	negative_squared,
	regional,
	segmented,
	other,
	parenthesized,
	sans_serif_bold_italic,
	sans_serif_bold,
	sans_serif_italic,
	sans_serif,
	script,
	small_capital,
	superscript,
	subscript,
	stretched,
	squared,
	tailed,
	wide
);

pub mod variant;

pub use variant::Variant;

pub use plain::try_as_plain;

/// Plain variant of Unicode symbols.
pub mod plain {
	/// Mapping of characters and its plain (upright, serifed) variant.
	///
	/// # Warnings
	///
	/// There is no Unicode standard for fonts.
	/// This is a best-effort mapping.
	///
	/// # Remarks
	///
	/// Plain symbols are mapped to themselves.
	/// This helps indicating if a symbols is considered in this best-effort collection.
	///
	/// In Unicode terms, this is a simple map since it maps `char` to `char`.
	///
	/// # Examples
	///
	/// Get the plan version of a character.
	/// ```
	/// use unicode_font::plain::PLAIN_MAP;
	/// let fancy_zero = '\u{1D7D8}'; // 𝟘
	/// assert_eq!(PLAIN_MAP.get(&fancy_zero).unwrap(), &'0');
	/// ```

	#[cfg(not(feature = "extension"))]
	pub const PLAIN_MAP: phf::Map<char, char> = include!("plain");
	#[cfg(feature = "extension")]
	pub const PLAIN_MAP: phf::Map<char, char> = include!("plain.extension");


	/// Returns the plain version of the character, if the character is supported.
	///
	/// # Examples
	///
	/// Get the plain version of a character.
	/// ```
	/// let fancy_zero = '\u{1D7D8}'; // 𝟘
	/// assert_eq!(unicode_font::try_as_plain(&fancy_zero).unwrap(), &'0');
	/// ```
	pub fn try_as_plain(s: &char) -> Option<&char> {
	    PLAIN_MAP.get(s)
	}

	#[cfg(test)]
	mod tests {
	    use super::*;

	    #[test]
	    /// Bold symbols are transformed into themselves.
	    fn double_plain_is_plain() {
	    	let plain: Vec<_> = PLAIN_MAP.values().cloned().collect();
	    	let double_plain: Vec<_> = plain.clone().into_iter().map(|c| try_as_plain(&c).cloned().unwrap()).collect(); 
	        assert_eq!(plain, double_plain);
	    }
	}
}