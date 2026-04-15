use pctrl_core::{PronunciationResult, Token};
use pctrl_dict::{Dictionary, Match};

include!("pinyin_map_phf.rs");

pub struct Engine {
    dict: Dictionary,
}

impl Engine {
    pub fn new(dict: Dictionary) -> Self {
        Self { dict }
    }

    pub fn convert(&self, text: &str) -> PronunciationResult {
        let char_indices: Vec<(usize, char)> = text.char_indices().collect();
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < char_indices.len() {
            let (byte_pos, ch) = char_indices[i];
            if !is_cjk(ch) {
                while i < char_indices.len() && !is_cjk(char_indices[i].1) {
                    i += 1;
                }
                let end_byte = if i < char_indices.len() {
                    char_indices[i].0
                } else {
                    text.len()
                };
                let surface = text[byte_pos..end_byte].to_string();
                tokens.push(Token {
                    pinyin: vec![surface.clone()],
                    surface,
                    source: "passthrough".into(),
                    strategy: "passthrough".into(),
                    priority: 0,
                    confidence: 1.0,
                });
                continue;
            }

            let substring = &text[byte_pos..];
            let matches = self.dict.lookup(substring);
            // Only consider matches that start at the current position (start == 0)
            let matches_at_pos: Vec<_> = matches.into_iter().filter(|m| m.start == 0).collect();
            if let Some(best) = pick_best(&self.dict, &matches_at_pos) {
                let entry = self.dict.entry(best.entry_index);
                let len = best.word.chars().count();
                tokens.push(Token {
                    surface: best.word.clone(),
                    pinyin: entry.pinyin.clone(),
                    source: entry.source.clone(),
                    strategy: "dictionary_exact".into(),
                    priority: entry.priority,
                    confidence: 1.0,
                });
                i += len;
            } else {
                let pinyin = char_to_pinyin(ch);
                tokens.push(Token {
                    surface: ch.to_string(),
                    pinyin: vec![pinyin],
                    source: "builtin".into(),
                    strategy: "fallback".into(),
                    priority: 100,
                    confidence: 0.8,
                });
                i += 1;
            }
        }

        PronunciationResult {
            text: text.into(),
            tokens,
        }
    }
}

fn is_cjk(ch: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&ch)
        || ('\u{3400}'..='\u{4DBF}').contains(&ch)
        || ('\u{20000}'..='\u{2A6DF}').contains(&ch)
}

// 优先匹配更长的词；长度相同时，priority 数值越高代表优先级越高
fn pick_best<'a>(dict: &'a Dictionary, matches: &'a [Match]) -> Option<&'a Match> {
    matches.iter().max_by(|a, b| {
        let len_a = a.word.chars().count();
        let len_b = b.word.chars().count();
        len_a.cmp(&len_b).then_with(|| {
            dict.entry(a.entry_index)
                .priority
                .cmp(&dict.entry(b.entry_index).priority)
        })
    })
}

fn char_to_pinyin(ch: char) -> String {
    match PINYIN_MAP.get(&ch) {
        Some(p) => (*p).into(),
        None => ch.to_string(),
    }
}
