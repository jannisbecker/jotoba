use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
