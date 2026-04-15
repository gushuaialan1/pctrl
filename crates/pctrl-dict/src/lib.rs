use pctrl_core::DictionaryEntry;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Dictionary {
    pub entries: Vec<DictionaryEntry>,
    pub trie: Trie,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_json_file<P: AsRef<Path>>(path: P, source: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut entries: Vec<DictionaryEntry> = serde_json::from_str(&content)?;
        for e in &mut entries {
            if e.source.is_empty() {
                e.source = source.into();
            }
        }
        let mut dict = Self::new();
        dict.entries = entries;
        dict.build_trie();
        Ok(dict)
    }

    pub fn from_entries(entries: Vec<DictionaryEntry>) -> Self {
        let mut dict = Self::new();
        dict.entries = entries;
        dict.build_trie();
        dict
    }

    pub fn build_trie(&mut self) {
        self.trie = Trie::new();
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.enabled {
                self.trie.insert(&entry.word, index);
            }
        }
    }

    pub fn lookup(&self, text: &str) -> Vec<Match> {
        self.trie.find_matches(text)
    }

    pub fn entry(&self, index: usize) -> &DictionaryEntry {
        &self.entries[index]
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    pub word: String,
    pub start: usize,
    pub end: usize,
    pub entry_index: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Debug, Default, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    entry_indexes: Vec<usize>,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str, entry_index: usize) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.entry_indexes.push(entry_index);
    }

    pub fn find_matches(&self, text: &str) -> Vec<Match> {
        let chars: Vec<char> = text.chars().collect();
        let mut matches = Vec::new();
        for i in 0..chars.len() {
            let mut node = &self.root;
            let mut j = i;
            while j < chars.len() {
                if let Some(next) = node.children.get(&chars[j]) {
                    node = next;
                    j += 1;
                    for entry_index in &node.entry_indexes {
                        matches.push(Match {
                            word: chars[i..j].iter().collect(),
                            start: i,
                            end: j,
                            entry_index: *entry_index,
                        });
                    }
                } else {
                    break;
                }
            }
        }
        matches
    }
}
