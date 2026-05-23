//! Indic script normalization — shared across Hindi, Bengali, and other Indic languages.
//!
//! Equivalent to Lucene's `IndicNormalizationFilter`. Normalizes:
//! - Nukta forms to base consonants
//! - Various zero-width characters removal
//! - Devanagari/Bengali canonical equivalences

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Normalizes common Indic script variations across Devanagari, Bengali, etc.
///
/// Handles zero-width joiners/non-joiners and canonical form normalization.
#[derive(Clone, Debug, Default)]
pub struct IndicNormalizationFilter;

impl IndicNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for IndicNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            match c {
                // Zero-width non-joiner (ZWNJ) - remove
                '\u{200C}' => { changed = true; }
                // Zero-width joiner (ZWJ) - remove
                '\u{200D}' => { changed = true; }
                // Devanagari: Nukta forms → base
                '\u{0958}' => { result.push('\u{0915}'); changed = true; } // ka + nukta → ka
                '\u{0959}' => { result.push('\u{0916}'); changed = true; } // kha + nukta → kha
                '\u{095A}' => { result.push('\u{0917}'); changed = true; } // ga + nukta → ga
                '\u{095B}' => { result.push('\u{091C}'); changed = true; } // ja + nukta → ja
                '\u{095C}' => { result.push('\u{0921}'); changed = true; } // dda + nukta → dda
                '\u{095D}' => { result.push('\u{0922}'); changed = true; } // ddha + nukta → ddha
                '\u{095E}' => { result.push('\u{092B}'); changed = true; } // pha + nukta → pha
                '\u{095F}' => { result.push('\u{092F}'); changed = true; } // ya + nukta → ya
                // Bengali: Nukta forms → base
                '\u{09DC}' => { result.push('\u{09A1}'); changed = true; } // rra → dda
                '\u{09DD}' => { result.push('\u{09A2}'); changed = true; } // rha → ddha
                '\u{09DF}' => { result.push('\u{09AF}'); changed = true; } // yya → ya
                // Devanagari/Bengali Avagraha - remove
                '\u{093D}' | '\u{09BD}' => { changed = true; }
                // Normalize Devanagari OM → OM
                '\u{0950}' => { result.push('\u{0950}'); }
                _ => { result.push(c); }
            }
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_zwnj() {
        let f = IndicNormalizationFilter::new();
        let mut token = Token::new("क\u{200C}ष", 0, 9, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "कष");
    }

    #[test]
    fn test_devanagari_nukta_form() {
        let f = IndicNormalizationFilter::new();
        let mut token = Token::new("\u{0958}", 0, 3, 0); // ka + nukta combined
        f.filter(&mut token);
        assert_eq!(token.term, "\u{0915}"); // plain ka
    }

    #[test]
    fn test_bengali_nukta_form() {
        let f = IndicNormalizationFilter::new();
        let mut token = Token::new("\u{09DC}", 0, 3, 0); // Bengali RRA
        f.filter(&mut token);
        assert_eq!(token.term, "\u{09A1}"); // Bengali DDA
    }

    #[test]
    fn test_no_change() {
        let f = IndicNormalizationFilter::new();
        let mut token = Token::new("नमस्ते", 0, 18, 0);
        let original = token.term.as_ref().to_string();
        f.filter(&mut token);
        assert_eq!(token.term.as_ref(), original);
    }
}
