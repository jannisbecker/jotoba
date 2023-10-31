use self::language::Language;

pub mod word;
pub mod language;
pub mod kanji;
pub mod misc;
pub mod part_of_speech;
pub mod dialect;
pub mod field;

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