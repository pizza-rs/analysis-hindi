<div align="center">

# 🇮🇳 pizza-analysis-hindi

**Hindi text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--hindi-blue)](https://github.com/pizza-rs/analysis-hindi)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Hindi/Devanagari language analysis with Indic script normalization, Hindi-specific
normalization, light stemming, and stop words.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `indic_normalization` | Normalize Indic script diacritics |
| TokenFilter | `hindi_normalization` | Hindi-specific character equivalences |
| TokenFilter | `hindi_stem` | Hindi light stemmer (suffix removal) |
| TokenFilter | `hindi_stop` | Hindi stop words (225 entries) |
| Analyzer | `hindi` | Full pipeline: lowercase → indic_norm → hindi_norm → stem → stop |

### Normalization

- **Indic**: Handles Nukta composites, visarga, and cross-script canonical equivalents
- **Hindi**: Normalizes Devanagari variants (chandrabindu, anusvara equivalents)

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_hindi::register_all(&mut factory);

let analyzer = factory.get_analyzer("hindi").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-hindi = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["hindi"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
