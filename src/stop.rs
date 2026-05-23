//! Hindi stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Hindi stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "अंदर",
    "अत",
    "अदि",
    "अप",
    "अपना",
    "अपनि",
    "अपनी",
    "अपने",
    "अभि",
    "अभी",
    "आदि",
    "आप",
    "इंहिं",
    "इंहें",
    "इंहों",
    "इतयादि",
    "इत्यादि",
    "इन",
    "इनका",
    "इन्हीं",
    "इन्हें",
    "इन्हों",
    "इस",
    "इसका",
    "इसकि",
    "इसकी",
    "इसके",
    "इसमें",
    "इसि",
    "इसी",
    "इसे",
    "उंहिं",
    "उंहें",
    "उंहों",
    "उन",
    "उनका",
    "उनकि",
    "उनकी",
    "उनके",
    "उनको",
    "उन्हीं",
    "उन्हें",
    "उन्हों",
    "उस",
    "उसके",
    "उसि",
    "उसी",
    "उसे",
    "एक",
    "एवं",
    "एस",
    "एसे",
    "ऐसे",
    "ओर",
    "और",
    "कइ",
    "कई",
    "कर",
    "करता",
    "करते",
    "करना",
    "करने",
    "करें",
    "कहते",
    "कहा",
    "का",
    "काफि",
    "काफ़ी",
    "कि",
    "किंहें",
    "किंहों",
    "कितना",
    "किन्हें",
    "किन्हों",
    "किया",
    "किर",
    "किस",
    "किसि",
    "किसी",
    "किसे",
    "की",
    "कुछ",
    "कुल",
    "के",
    "को",
    "कोइ",
    "कोई",
    "कोन",
    "कोनसा",
    "कौन",
    "कौनसा",
    "गया",
    "घर",
    "जब",
    "जहाँ",
    "जहां",
    "जा",
    "जिंहें",
    "जिंहों",
    "जितना",
    "जिधर",
    "जिन",
    "जिन्हें",
    "जिन्हों",
    "जिस",
    "जिसे",
    "जीधर",
    "जेसा",
    "जेसे",
    "जैसा",
    "जैसे",
    "जो",
    "तक",
    "तब",
    "तरह",
    "तिंहें",
    "तिंहों",
    "तिन",
    "तिन्हें",
    "तिन्हों",
    "तिस",
    "तिसे",
    "तो",
    "था",
    "थि",
    "थी",
    "थे",
    "दबारा",
    "दवारा",
    "दिया",
    "दुसरा",
    "दुसरे",
    "दूसरे",
    "दो",
    "द्वारा",
    "न",
    "नहिं",
    "नहीं",
    "ना",
    "निचे",
    "निहायत",
    "नीचे",
    "ने",
    "पर",
    "पहले",
    "पुरा",
    "पूरा",
    "पे",
    "फिर",
    "बनि",
    "बनी",
    "बहि",
    "बही",
    "बहुत",
    "बाद",
    "बाला",
    "बिलकुल",
    "भि",
    "भितर",
    "भी",
    "भीतर",
    "मगर",
    "मानो",
    "मे",
    "में",
    "यदि",
    "यह",
    "यहाँ",
    "यहां",
    "यहि",
    "यही",
    "या",
    "यिह",
    "ये",
    "रखें",
    "रवासा",
    "रहा",
    "रहे",
    "ऱ्वासा",
    "लिए",
    "लिये",
    "लेकिन",
    "व",
    "वगेरह",
    "वरग",
    "वर्ग",
    "वह",
    "वहाँ",
    "वहां",
    "वहिं",
    "वहीं",
    "वाले",
    "वुह",
    "वे",
    "वग़ैरह",
    "संग",
    "सकता",
    "सकते",
    "सबसे",
    "सभि",
    "सभी",
    "साथ",
    "साबुत",
    "साभ",
    "सारा",
    "से",
    "सो",
    "हि",
    "ही",
    "हुअ",
    "हुआ",
    "हुइ",
    "हुई",
    "हुए",
    "हे",
    "हें",
    "है",
    "हैं",
    "हो",
    "होता",
    "होति",
    "होती",
    "होते",
    "होना",
    "होने",
    ];
    words.iter().copied().collect()
});

/// Removes Hindi stop words from the token stream.
#[derive(Clone, Debug)]
pub struct HindiStopFilter {
    stop_words: HashSet<String>,
}

impl Default for HindiStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl HindiStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for HindiStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 225);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = HindiStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = HindiStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = HindiStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
