# Changelog

## [0.1.0] - 2026-04-15

### Added
- Initial release of pctrl (Pronunciation Control)
- 7-crate Rust workspace architecture
- 3-tier dictionary system based on CC-CEDICT (common ~105k entries, history ~4.9k entries, history_core manual corrections)
- CLI commands: convert, analyze, init, doctor, dict validate/list, benchmark, export
- Forward maximum-match engine with Trie and PHF single-character fallback
- Support for plain, json, segmented, bert-vits2, gpt-sovits, and generic_phoneme_json output formats
