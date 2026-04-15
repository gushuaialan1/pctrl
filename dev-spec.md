# PCTRL Development Specification

## Project Overview

Build a Rust CLI tool `pctrl` (Pronunciation Control) that acts as a TTS preprocessing layer for Chinese text. Phase 1 targets **historical narration videos** with high-risk mispronunciation terms.

## Architecture (Workspace)

```
crates/
  pctrl-core      # Core data structures (Token, Candidate, Decision, Result)
  pctrl-dict      # Dictionary loading and trie-based indexing
  pctrl-segment   # Text segmentation (Chinese, punctuation, mixed alphanumeric)
  pctrl-engine    # Matching engine (max-match, priority resolution, fallback)
  pctrl-output    # Output formatters (plain, json, debug)
  pctrl-config    # Configuration loading (built-in < user < project < CLI)
  pctrl-cli       # CLI binary with clap subcommands
```

## Milestone 1-2 Scope

Implement enough for `pctrl convert "TEXT"` to work end-to-end with JSON output.

### pctrl-core

- `Token`: surface text, pinyin Vec<String>, source, strategy, priority, confidence
- `PronunciationResult`: original text, tokens Vec<Token>
- `DictionaryEntry`: word, pinyin Vec<String>, category, priority, source, tags, enabled

### pctrl-dict

- Load dictionaries from JSON files under `dictionaries/history/*.json`
- Build a `DoubleArrayTrie` or simple `HashMap`-based prefix trie for forward maximum matching
- Support priority per entry and source-level grouping
- MVP: load a static `history_core.json` with ~20 high-risk terms

### pctrl-segment

- Split input into segments preserving punctuation and non-CJK characters
- Output: Vec<String> segments ready for matching

### pctrl-engine

- Forward maximum match using the trie
- On overlap/ conflict: longer wins, then higher priority wins
- Fallback: character-by-character using a minimal built-in pinyin map (embedded PHF map for ~500 common characters)
- Returns `PronunciationResult`

### pctrl-output

- `plain`: space-separated pinyin
- `json`: structured `PronunciationResult`
- `debug`: internal representation (future)

### pctrl-config

- Load TOML config from `~/.config/pctrl/config.toml` and `./.pctrl/config.toml`
- Config structs: EngineConfig, PriorityConfig, OutputConfig, DictionariesConfig
- CLI flags override config values

### pctrl-cli

Subcommands for this milestone:
- `convert <TEXT>`: convert to pinyin, `--format plain|json`, `--stdin` support
- `init`: create `./.pctrl/config.toml` with defaults
- `doctor`: check config paths and dictionary validity

### Dictionary Content (MVP)

Create `dictionaries/history/history_core.json` with at least these entries:
- 单于 chan2 yu2
- 阏氏 yan1 zhi1
- 可汗 ke4 han2
- 吐蕃 tu3 bo1
- 龟兹 qiu1 ci2
- 于阗 yu2 tian2
- 长孙 zhang3 sun1
- 尉迟 yu4 chi2
- 仆射 pu2 ye4
- 给事中 ji3 shi4 zhong1
- 谥号 shi4 hao4
- 庙号 miao4 hao4
- 觊觎 ji4 yu2
- 龃龉 ju3 yu3
- 睥睨 pi4 ni4
- 桎梏 zhi4 gu4
- 纨绔 wan2 ku4
- 龟裂 jun1 lie4
- 角色 jue2 se4
- 氛围 fen1 wei2

Each entry follows the JSON schema defined in core.

### Tests

- Unit tests for trie matching and fallback
- Integration tests for the 20 high-risk terms
- CLI tests using `assert_cmd`

### Non-Goals for Milestone 1-2

- `analyze`, `dict`, `benchmark`, `export` subcommands
- Project-level dictionary editing
- TTS-specific export formats
- Complex multi-character fallback beyond single-char map

## Coding Rules

1. No emojis in code or comments.
2. Use `thiserror` for library errors, `anyhow` for CLI.
3. All public functions must have doc comments.
4. Keep CLI thin; business logic lives in crates.
5. Use `serde` + `toml` for config, `serde_json` for dictionaries.
6. Single-character pinyin fallback map must be embedded at compile time using `phf_macros`.

## Acceptance Criteria

- `cargo build` succeeds
- `cargo test` passes
- `pctrl convert "单于夜遁逃"` outputs `chan2 yu2 ye4 dun4 tao2`
- `pctrl convert "长孙无忌"` outputs `zhang3 sun1 wu2 ji4`
- JSON output contains tokens with source = "history_core" for matched terms
- `pctrl doctor` reports dictionary count and config paths
