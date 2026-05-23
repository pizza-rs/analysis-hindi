use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::indic::IndicNormalizationFilter;
use crate::normalization::HindiNormalizationFilter;
use crate::stem::HindiStemFilter;
use crate::stop::HindiStopFilter;

/// Register all Hindi analysis components.
///
/// Registers:
/// - `"hindi"` analyzer (indic_normalization → hindi_normalization → stop → stem)
/// - `"indic_normalization"` token filter
/// - `"hindi_normalization"` token filter
/// - `"hindi_stem"` token filter
/// - `"hindi_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("indic_normalization", Box::new(IndicNormalizationFilter::new()));
    factory.register_token_filter("hindi_normalization", Box::new(HindiNormalizationFilter::new()));
    factory.register_token_filter("hindi_stem", Box::new(HindiStemFilter::new()));
    factory.register_token_filter("hindi_stop", Box::new(HindiStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(IndicNormalizationFilter::new()),
        Box::new(HindiNormalizationFilter::new()),
        Box::new(HindiStopFilter::new()),
        Box::new(HindiStemFilter::new()),
    ];

    factory.register_analyzer(
        "hindi",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("hindi_normalization").is_some());
        assert!(factory.get_token_filter("hindi_stem").is_some());
        assert!(factory.get_token_filter("hindi_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("hindi").is_some());
    }
}
