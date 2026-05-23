//! Comprehensive tests for pizza-analysis-hindi.

use pizza_analysis_hindi::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// IndicNormalizationFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn indic_normalization_construction() {
    let _f = IndicNormalizationFilter::new();
}

#[test]
fn indic_normalization_devanagari() {
    let f = IndicNormalizationFilter::new();
    let mut token = make_token("हिन्दी");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn indic_normalization_ascii_passthrough() {
    let f = IndicNormalizationFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn indic_normalization_empty() {
    let f = IndicNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// HindiNormalizationFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn hindi_normalization_construction() {
    let _f = HindiNormalizationFilter::new();
}

#[test]
fn hindi_normalization_chandra_bindu() {
    let f = HindiNormalizationFilter::new();
    // Chandrabindu normalization
    let mut token = make_token("हँसना");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn hindi_normalization_nukta() {
    let f = HindiNormalizationFilter::new();
    // Nukta normalization
    let mut token = make_token("ज़मीन");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn hindi_normalization_ascii_passthrough() {
    let f = HindiNormalizationFilter::new();
    let mut token = make_token("world");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "world");
}

#[test]
fn hindi_normalization_empty() {
    let f = HindiNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// HindiStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = HindiStemFilter::new();
}

#[test]
fn stem_plural_suffix() {
    let f = HindiStemFilter::new();
    // "लड़कों" (boys) → stem
    let mut token = make_token("लड़कों");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_postposition() {
    let f = HindiStemFilter::new();
    // "किताबों" (books) → stem
    let mut token = make_token("किताबों");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_suffix() {
    let f = HindiStemFilter::new();
    let mut token = make_token("खेलना");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = HindiStemFilter::new();
    let mut token = make_token("में");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = HindiStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// HindiStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = HindiStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = HindiStopFilter::new();
    let stop_words = ["और", "के", "का", "है", "में", "की", "को", "से", "एक", "यह"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = HindiStopFilter::new();
    let content_words = ["किताब", "विद्यालय", "शहर", "पुस्तक"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = HindiStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("indic_normalization").is_some());
    assert!(factory.get_token_filter("hindi_normalization").is_some());
    assert!(factory.get_token_filter("hindi_stem").is_some());
    assert!(factory.get_token_filter("hindi_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("hindi").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hindi").unwrap();
    let mut input = String::from("भारत एक बड़ा देश है");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hindi").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hindi").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
