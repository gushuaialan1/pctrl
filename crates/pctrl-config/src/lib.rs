use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    #[serde(default = "default_max_match")]
    pub max_match: bool,
    #[serde(default = "default_preserve_punctuation")]
    pub preserve_punctuation: bool,
    #[serde(default = "default_fallback")]
    pub fallback: String,
    #[serde(default = "default_unknown_word_policy")]
    pub unknown_word_policy: String,
}

fn default_max_match() -> bool { true }
fn default_preserve_punctuation() -> bool { true }
fn default_fallback() -> String { "builtin".into() }
fn default_unknown_word_policy() -> String { "char_by_char".into() }

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_match: true,
            preserve_punctuation: true,
            fallback: "builtin".into(),
            unknown_word_policy: "char_by_char".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityConfig {
    #[serde(default = "default_user_dict")]
    pub user_dict: i32,
    #[serde(default = "default_history_dict")]
    pub history_dict: i32,
    #[serde(default = "default_common_misread")]
    pub common_misread: i32,
    #[serde(default = "default_rule_engine")]
    pub rule_engine: i32,
    #[serde(default = "default_builtin")]
    pub builtin: i32,
}

fn default_user_dict() -> i32 { 1000 }
fn default_history_dict() -> i32 { 900 }
fn default_common_misread() -> i32 { 800 }
fn default_rule_engine() -> i32 { 500 }
fn default_builtin() -> i32 { 100 }

impl Default for PriorityConfig {
    fn default() -> Self {
        Self {
            user_dict: 1000,
            history_dict: 900,
            common_misread: 800,
            rule_engine: 500,
            builtin: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub default_format: String,
    #[serde(default = "default_true")]
    pub show_source: bool,
    #[serde(default = "default_true")]
    pub show_confidence: bool,
}

fn default_format() -> String { "plain".into() }
fn default_true() -> bool { true }

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_format: "plain".into(),
            show_source: true,
            show_confidence: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionariesConfig {
    #[serde(default)]
    pub enabled: Vec<String>,
    #[serde(default)]
    pub paths: Vec<PathBuf>,
}

impl Default for DictionariesConfig {
    fn default() -> Self {
        Self {
            enabled: vec![
                "cc_cedict_common".into(),
                "cc_cedict_history".into(),
                "history_core".into(),
            ],
            paths: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub engine: EngineConfig,
    #[serde(default)]
    pub priority: PriorityConfig,
    #[serde(default)]
    pub dictionaries: DictionariesConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

impl Config {
    pub fn load(config_path: Option<&std::path::Path>) -> anyhow::Result<Self> {
        let mut config = Self::default();
        if let Some(p) = config_path {
            if p.exists() {
                let content = std::fs::read_to_string(p)?;
                config = toml::from_str(&content)?;
            }
        }
        Ok(config)
    }

    pub fn default_toml() -> String {
        r#"[engine]
max_match = true
preserve_punctuation = true
fallback = "builtin"
unknown_word_policy = "char_by_char"

[priority]
user_dict = 1000
history_dict = 900
common_misread = 800
rule_engine = 500
builtin = 100

[dictionaries]
enabled = ["cc_cedict_common", "cc_cedict_history", "history_core"]

[output]
default_format = "plain"
show_source = true
show_confidence = true
"#
        .into()
    }
}
