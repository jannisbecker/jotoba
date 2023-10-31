use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub kanji: Vec<Kanji>,
}

#[derive(Serialize, Deserialize)]
pub struct Kanji {
    pub literal: String,
    // can be missing despite not being optional in Jotoba source
    pub meanings: Option<Vec<String>>,
    pub grade: Option<u8>,
    pub stroke_count: u8,
    pub frequency: Option<u16>,
    pub jlpt: Option<u8>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<Vec<String>>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub parts: Option<Vec<String>>,
    pub radical: String,
    pub stroke_frames: Option<String>,
}
