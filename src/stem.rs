use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Hindi stemmer that removes common inflectional suffixes.
/// Based on lightweight approach suitable for Hindi information retrieval.
#[derive(Clone, Debug, Default)]
pub struct HindiStemFilter;

impl HindiStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for HindiStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let char_count = text.chars().count();
        if char_count < 4 {
            return (false, None);
        }

        let stemmed = stem_hindi(text, char_count);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_hindi(word: &str, len: usize) -> String {
    let chars: Vec<char> = word.chars().collect();

    // Try longest suffixes first (5 chars)
    if len > 6 {
        let suffix5: String = chars[len - 5..].iter().collect();
        match suffix5.as_str() {
            "ाइयों" | "ाओंने" => return chars[..len - 5].iter().collect(),
            _ => {}
        }
    }

    // 4-char suffixes
    if len > 5 {
        let suffix4: String = chars[len - 4..].iter().collect();
        match suffix4.as_str() {
            "ाइयाँ" | "ाइयों" | "ियोंने" => return chars[..len - 4].iter().collect(),
            _ => {}
        }
    }

    // 3-char suffixes
    if len > 4 {
        let suffix3: String = chars[len - 3..].iter().collect();
        match suffix3.as_str() {
            "ियाँ" | "ियों" | "ाओं" | "ाएँ" | "ाईं" | "ाएं"
            | "ोंने" | "ेंगे" | "ेंगी" => return chars[..len - 3].iter().collect(),
            _ => {}
        }
    }

    // 2-char suffixes
    if len > 3 {
        let suffix2: String = chars[len - 2..].iter().collect();
        match suffix2.as_str() {
            "ों" | "ें" | "ाँ" | "ीं" | "ाई" | "ाए" | "ने"
            | "नी" | "ना" | "ते" | "ती" | "ता" | "ीय" | "ेगा"
            | "ेगी" | "ाक" | "ाप" | "ाव" | "कर" => {
                return chars[..len - 2].iter().collect();
            }
            _ => {}
        }
    }

    // 1-char suffixes
    if len > 3 {
        let last = chars[len - 1];
        match last {
            'ा' | 'े' | 'ी' | 'ो' | 'ं' | 'ँ' => {
                return chars[..len - 1].iter().collect();
            }
            _ => {}
        }
    }

    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_plural() {
        let filter = HindiStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("लड़कों"),
            start_offset: 0,
            end_offset: 15,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        // Should remove the plural suffix
        assert!(token.term.as_ref().len() < "लड़कों".len());
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = HindiStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("घर"),
            start_offset: 0,
            end_offset: 6,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "घर");
    }
}
