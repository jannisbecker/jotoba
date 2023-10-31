use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Dialect {
    Brazilian,
    Hokkaido,
    Kansai,
    Kantou,
    Kyoto,
    Kyuushuu,
    Nagano,
    Osaka,
    Ryuukyuu,
    Touhoku,
    Tosa,
    Tsugaru,
}
