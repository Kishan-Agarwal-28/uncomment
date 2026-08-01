use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct StringDelimiter {
    pub open: String,
    pub close: String,
    #[serde(default = "default_true")]
    pub escape: bool,
    #[serde(default)]
    pub multiline: bool,

    #[serde(default)]
    pub char_literal: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageRules {
    pub name: String,
    pub extensions: Vec<String>,
    #[serde(default)]
    pub line_comments: Vec<String>,
    #[serde(default)]
    pub block_comments: Vec<[String; 2]>,
    #[serde(default)]
    pub strings: Vec<StringDelimiter>,

    #[serde(default)]
    pub raw_strings: bool,
}

#[derive(Debug, Deserialize)]
pub struct LanguageConfig {
    pub language: Vec<LanguageRules>,
}

pub fn build_index(rules: Vec<LanguageRules>) -> HashMap<String, LanguageRules> {
    let mut map = HashMap::new();
    for lang in rules {
        for ext in &lang.extensions {
            map.insert(ext.clone(), lang.clone());
        }
    }
    map
}

pub fn builtin_config() -> LanguageConfig {
    let src = include_str!("./languages.toml");
    toml::from_str(src).expect("built-in languages.toml is malformed")
}
