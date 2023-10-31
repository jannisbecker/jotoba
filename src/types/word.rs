use serde::{Deserialize, Serialize};

use crate::types::kanji::Kanji;
use crate::types::language::Language;
use crate::types::misc::Misc;
use crate::types::part_of_speech::PartOfSpeech;

use super::dialect::Dialect;
use super::field::Field;

/// The API response struct for a word search
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub kanji: Vec<Kanji>,
    pub words: Vec<Word>,
}

/// Represents a single Word result with 1 (main) Japanese reading and n glosses
#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub reading: Reading,
    pub common: bool,
    pub senses: Vec<Sense>,
    pub alt_readings: Option<Vec<Reading>>,
    pub audio: Option<String>,
    pub pitch: Option<Vec<PitchPart>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reading {
    pub kana: String,
    pub kanji: Option<String>,
    pub furigana: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sense {
    pub glosses: Vec<String>,
    // can be missing despite not being optional in Jotoba source
    pub pos: Option<Vec<PartOfSpeech>>,
    pub language: Language,
    pub dialect: Option<Dialect>,
    pub field: Option<Field>,
    pub information: Option<String>,
    pub antonym: Option<String>,
    pub misc: Option<Misc>,
    pub xref: Option<String>,
}

/// A single, owned part of a whole pitch entry for a word
#[derive(Debug, Serialize, Deserialize)]
pub struct PitchPart {
    pub part: String,
    pub high: bool,
}
