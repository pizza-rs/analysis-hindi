# pizza-analysis-hindi

Hindi language analysis with Indic normalization, Hindi-specific normalization, stemming, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `indic_normalization` | Token Filter | Shared Indic normalization — nukta forms, zero-width characters |
| `hindi_normalization` | Token Filter | Hindi-specific character normalization |
| `hindi_stem` | Token Filter | Hindi light stemmer — removes common suffixes |
| `hindi_stop` | Token Filter | Hindi stop words filter (225 words) |
| `hindi` | Analyzer | Full pipeline: indic_normalization → hindi_normalization → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "hindi"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["indic_normalization", "hindi_normalization", "hindi_stem", "hindi_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
