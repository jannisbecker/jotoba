use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Language {
    English,
    German,
    Russain,
    Spanish,
    Swedish,
    French,
    Dutch,
    Hungarian,
    Slovenian,
    Japanese,
}