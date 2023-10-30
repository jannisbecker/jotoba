use serde::{Deserialize, Serialize};

pub struct WordsQueryOptions {
    pub language: Language,
    pub no_english: bool,
}

impl Default for WordsQueryOptions {
    fn default() -> Self {
        Self {
            language: Language::English,
            no_english: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WordsResponse {
    pub words: Vec<Word>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub reading: WordReading,
    pub common: bool,
    pub senses: Vec<WordSense>,
    pub pitch: Option<Vec<WordPitchPart>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordReading {
    pub kana: Option<String>,
    pub kanji: Option<String>,
    pub furigana: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordSense {
    pub glosses: Vec<String>,
    // pos: Vec<String>, unclear dynamic structure in response
    pub language: Language,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Language {
    English,
    German,
    Spanish,
    Russian,
    Swedish,
    French,
    Dutch,
    Hungarian,
    Slovenian,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordPitchPart {
    pub part: String,
    pub high: bool,
}
