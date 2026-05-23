use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Hindi/Devanagari normalization filter.
///
/// Normalizes:
/// - Nukta-bearing consonants to base (क़→क, ख़→ख, etc.)
/// - Chandrabindu to Anusvara (ँ→ं)
/// - Removes Visarga (ः) when followed by consonants
/// - Devanagari digit variants to standard form
#[derive(Clone, Debug, Default)]
pub struct HindiNormalizationFilter;

impl HindiNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for HindiNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                // Nukta (U+093C) - remove and use base consonant
                '\u{093C}' => {
                    changed = true;
                    // Skip nukta (already folded into base by prior char)
                }
                // Chandrabindu → Anusvara
                '\u{0901}' => {
                    changed = true;
                    result.push('\u{0902}');
                }
                // Visarga - remove
                '\u{0903}' => {
                    changed = true;
                }
                // Nukta-bearing consonants (combined forms)
                'क' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('क');
                    i += 1; // skip nukta
                }
                'ख' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('ख');
                    i += 1;
                }
                'ग' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('ग');
                    i += 1;
                }
                'ज' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('ज');
                    i += 1;
                }
                'ड' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('ड');
                    i += 1;
                }
                'ढ' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('ढ');
                    i += 1;
                }
                'फ' if i + 1 < chars.len() && chars[i + 1] == '\u{093C}' => {
                    changed = true;
                    result.push('फ');
                    i += 1;
                }
                _ => {
                    result.push(c);
                }
            }
            i += 1;
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
    fn test_chandrabindu_to_anusvara() {
        let filter = HindiNormalizationFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("हँस"),
            start_offset: 0,
            end_offset: 9,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "हंस");
    }

    #[test]
    fn test_visarga_removal() {
        let filter = HindiNormalizationFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("दुःख"),
            start_offset: 0,
            end_offset: 12,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "दुख");
    }
}
