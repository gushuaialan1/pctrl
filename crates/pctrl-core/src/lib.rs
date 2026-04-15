use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub pinyin: Vec<String>,
    pub category: Option<String>,
    pub priority: i32,
    pub source: String,
    pub common_errors: Option<Vec<String>>,
    pub notes: Option<String>,
    pub enabled: bool,
    pub tags: Option<Vec<String>>,
    pub version: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub surface: String,
    pub pinyin: Vec<String>,
    pub source: String,
    pub strategy: String,
    pub priority: i32,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PronunciationResult {
    pub text: String,
    pub tokens: Vec<Token>,
}

impl PronunciationResult {
    pub fn to_plain(&self) -> String {
        self.tokens
            .iter()
            .flat_map(|t| t.pinyin.iter().cloned())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_segmented(&self) -> String {
        let surfaces: Vec<_> = self.tokens.iter().map(|t| t.surface.clone()).collect();
        let pinyins: Vec<_> = self
            .tokens
            .iter()
            .map(|t| t.pinyin.join(" "))
            .collect();
        format!("{}\n{}", surfaces.join("/"), pinyins.join(" / "))
    }
}
